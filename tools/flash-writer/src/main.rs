use std::{path::PathBuf, process};

use clap::Parser;
use flash_writer::{flash, FlashOptions};

#[derive(Parser)]
#[command(
    name = "flash_writer",
    bin_name = "flash_writer",
    about = "Install Spresense SPK packages over UART",
    override_usage = "flash_writer [-s] -c <port> [-d] [-b <baud>] <spk> [<spk>...]"
)]
struct Cli {
    /// Use serial transport (accepted for compatibility; always active)
    #[arg(short = 's', action = clap::ArgAction::SetTrue)]
    serial: bool,

    /// Serial port device (e.g. /dev/ttyUSB0)
    #[arg(short = 'c', long = "serial-port", value_name = "PORT", required = true)]
    port: String,

    /// Pulse DTR to auto-reset the board before flashing
    #[arg(short = 'd', long = "dtr-reset")]
    dtr_reset: bool,

    /// Switch to this baud rate for the XMODEM transfer phase
    #[arg(short = 'b', long = "xmodem-baudrate", value_name = "BAUD")]
    xmodem_baud: Option<u32>,

    /// SPK file(s) to install
    #[arg(value_name = "spk", required = true)]
    packages: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let pkg_refs: Vec<&std::path::Path> = cli.packages.iter().map(|p| p.as_path()).collect();

    let opts = FlashOptions {
        port: &cli.port,
        packages: &pkg_refs,
        dtr_reset: cli.dtr_reset,
        xmodem_baud: cli.xmodem_baud,
        set_bootable: true,
        reboot: true,
    };

    if let Err(e) = flash(&opts) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
