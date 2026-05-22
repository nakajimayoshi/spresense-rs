use std::{
    env, fs,
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail, Context, Result};
use cargo_metadata::{Message, MetadataCommand};
use clap::{ArgGroup, Parser};

const DEFAULT_TARGET: &str = "thumbv7em-none-eabihf";
const DEFAULT_PORT: &str = "/dev/ttyUSB0";

#[derive(Parser, Debug)]
#[command(name = "cargo-spresense-flash", bin_name = "cargo")]
enum CargoCli {
    SpresenseFlash(Cli),
}

#[derive(clap::Args, Debug)]
#[command(
    version,
    about = "Build and flash a Rust binary to a Sony Spresense board",
    group(
        ArgGroup::new("artifact").required(true).multiple(false).args(["bin", "example"])
    )
)]
struct Cli {
    /// Binary target to build and flash
    #[arg(long, value_name = "NAME")]
    bin: Option<String>,

    /// Example target to build and flash
    #[arg(long, value_name = "NAME")]
    example: Option<String>,

    /// Build in release mode
    #[arg(long)]
    release: bool,

    /// Build with the given profile (overrides --release)
    #[arg(long, value_name = "NAME")]
    profile: Option<String>,

    /// Cargo features to activate (comma-separated) [default: "rt"]
    #[arg(long, value_name = "FEATURES")]
    features: Option<String>,

    /// Target triple
    #[arg(long, value_name = "TRIPLE", default_value = DEFAULT_TARGET)]
    target: String,

    /// Disable default Cargo features
    #[arg(long)]
    no_default_features: bool,

    /// Path to Cargo.toml of the project to build
    #[arg(long, value_name = "PATH")]
    manifest_path: Option<PathBuf>,

    /// Serial port device
    #[arg(long, value_name = "DEV", env = "SPRESENSE_PORT", default_value = DEFAULT_PORT)]
    port: String,

    /// XMODEM-phase baud rate (0 = stay at 115200 throughout)
    #[arg(long, value_name = "RATE", default_value_t = 0)]
    baud: u32,

    /// mkspk -c core selector
    #[arg(long, value_name = "N", default_value_t = 2)]
    core: u32,

    /// Override the SPK install name stored in the package header
    #[arg(long, value_name = "NAME")]
    name: Option<String>,

    /// Skip cargo build and reuse an existing artifact on disk
    #[arg(long)]
    skip_build: bool,
}

fn main() -> Result<()> {
    let CargoCli::SpresenseFlash(cli) = CargoCli::parse();

    let artifact_name = cli
        .bin
        .as_deref()
        .or(cli.example.as_deref())
        .expect("clap ArgGroup guarantees one is set");
    let spk_name = cli.name.as_deref().unwrap_or(artifact_name);

    // Staging directory for stripped ELF and SPK.
    let stage_dir = stage_dir(artifact_name, &cli)?;
    fs::create_dir_all(&stage_dir)
        .with_context(|| format!("creating staging dir {}", stage_dir.display()))?;

    let elf_src = if cli.skip_build {
        // Reconstruct the expected artifact path from cargo metadata.
        find_existing_artifact(&cli, artifact_name)?
    } else {
        build(&cli, artifact_name)?
    };

    let staged_elf = stage_dir.join(format!("{}.elf", spk_name));
    let staged_spk = stage_dir.join(format!("{}.spk", spk_name));

    fs::copy(&elf_src, &staged_elf)
        .with_context(|| format!("copying ELF to {}", staged_elf.display()))?;

    strip_debug(&staged_elf);

    package(&staged_elf, spk_name, &staged_spk, cli.core)?;

    flash(&cli.port, cli.baud, &staged_spk)?;

    Ok(())
}

fn stage_dir(name: &str, cli: &Cli) -> Result<PathBuf> {
    // Place under <manifest>/target/spresense-flash/<name>/.
    // Fall back to cwd/target if metadata fails.
    let mut mcmd = MetadataCommand::new();
    if let Some(ref mp) = cli.manifest_path {
        mcmd.manifest_path(mp);
    }
    let target_dir = mcmd
        .no_deps()
        .exec()
        .map(|m| m.target_directory.into_std_path_buf())
        .unwrap_or_else(|_| PathBuf::from("target"));
    Ok(target_dir.join("spresense-flash").join(name))
}

fn build(cli: &Cli, artifact_name: &str) -> Result<PathBuf> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut cmd = Command::new(&cargo);
    cmd.arg("build")
        .arg("--target")
        .arg(&cli.target)
        .args(["--message-format", "json-render-diagnostics"]);

    if let Some(ref p) = cli.profile {
        cmd.args(["--profile", p]);
    } else if cli.release {
        cmd.arg("--release");
    }

    let features = cli.features.as_deref().unwrap_or("rt");
    cmd.args(["--features", features]);

    if cli.no_default_features {
        cmd.arg("--no-default-features");
    }

    if let Some(ref mp) = cli.manifest_path {
        cmd.arg("--manifest-path").arg(mp);
    }

    if cli.bin.is_some() {
        cmd.args(["--bin", artifact_name]);
    } else {
        cmd.args(["--example", artifact_name]);
    }

    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().context("failed to spawn cargo build")?;
    let stdout = child.stdout.take().unwrap();

    let mut elf_path: Option<PathBuf> = None;
    let expected_kind = if cli.bin.is_some() { "bin" } else { "example" };

    for message in Message::parse_stream(BufReader::new(stdout)) {
        match message.context("reading cargo JSON stream")? {
            Message::CompilerArtifact(art)
                if art.target.name == artifact_name
                    && art.target.kind.iter().any(|k| k == expected_kind) =>
            {
                if let Some(exe) = art.executable {
                    elf_path = Some(exe.into_std_path_buf());
                }
            }
            _ => {}
        }
    }

    let status = child.wait().context("cargo build did not exit cleanly")?;
    if !status.success() {
        bail!("cargo build failed (exit {})", status);
    }

    elf_path.ok_or_else(|| {
        anyhow!("cargo build succeeded but no executable artifact found for '{artifact_name}'")
    })
}

fn find_existing_artifact(cli: &Cli, artifact_name: &str) -> Result<PathBuf> {
    let mut mcmd = MetadataCommand::new();
    if let Some(ref mp) = cli.manifest_path {
        mcmd.manifest_path(mp);
    }
    let meta = mcmd.no_deps().exec().context("cargo metadata failed")?;

    let profile_dir = if let Some(ref p) = cli.profile {
        p.as_str()
    } else if cli.release {
        "release"
    } else {
        "debug"
    };

    let subdir = if cli.bin.is_some() { "" } else { "examples" };
    let base: PathBuf = meta
        .target_directory
        .as_std_path()
        .join(&cli.target)
        .join(profile_dir);
    let path = if subdir.is_empty() {
        base.join(artifact_name)
    } else {
        base.join(subdir).join(artifact_name)
    };

    if !path.exists() {
        bail!(
            "artifact not found (--skip-build): {}\nRun without --skip-build to build first.",
            path.display()
        );
    }
    Ok(path)
}

fn strip_debug(elf: &Path) {
    let status = Command::new("arm-none-eabi-strip")
        .args(["-d", &elf.to_string_lossy()])
        .status();
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => eprintln!("warning: arm-none-eabi-strip exited {s} — continuing anyway"),
        Err(_) => {
            eprintln!("warning: arm-none-eabi-strip not found — skipping strip");
        }
    }
}

fn package(elf: &Path, name: &str, spk: &Path, core: u32) -> Result<()> {
    eprintln!(
        "   Packaging {} -> {} (mkspk -c {})",
        elf.file_name().unwrap_or_default().to_string_lossy(),
        spk.file_name().unwrap_or_default().to_string_lossy(),
        core,
    );
    let status = Command::new("mkspk")
        .args([
            "-c",
            &core.to_string(),
            &elf.to_string_lossy(),
            name,
            &spk.to_string_lossy(),
        ])
        .status()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow!("mkspk not found on PATH — run: cargo install --path tools/mkspk")
            } else {
                anyhow!("failed to spawn mkspk: {e}")
            }
        })?;

    if !status.success() {
        bail!("mkspk failed (exit {})", status);
    }
    Ok(())
}

fn flash(port: &str, baud: u32, spk: &Path) -> Result<()> {
    let baud_str = baud.to_string();
    eprintln!(
        "   Flashing {} @ {} baud  (flash_writer -s -c {port} -d)",
        spk.file_name().unwrap_or_default().to_string_lossy(),
        if baud == 0 { "115200" } else { &baud_str },
    );

    let mut cmd = Command::new("flash_writer");
    cmd.args(["-s", "-c", port, "-d"]);
    if baud != 0 {
        cmd.args(["-b", &baud_str]);
    }
    cmd.arg(spk);

    // Inherit stdio so the user sees flash_writer's progress directly.
    let status = cmd.status().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow!("flash_writer not found on PATH — run: cargo install --path tools/flash-writer")
        } else {
            anyhow!("failed to spawn flash_writer: {e}")
        }
    })?;
    if !status.success() {
        bail!("flash_writer failed (exit {})", status);
    }

    eprintln!("   Done. Board rebooting into M0P.");
    Ok(())
}
