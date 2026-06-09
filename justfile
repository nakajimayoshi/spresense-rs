_default:
    @just --list

# Common work for PAC generation and updating
_generate_pac pac-dir:
    rustfmt svd-out/lib.rs
    form -f -i svd-out/lib.rs -o svd-out/src/
    rm -rf {{ pac-dir }}/src
    cp -r svd-out/src/ {{ pac-dir }}
    cp svd-out/device.x {{ pac-dir }}
    rm -rf svd-out
    cargo fmt -p {{ pac-dir }}

# Regenerate and update chiptool PAC
chiptool: _svdtools
    mkdir -p svd-out
    chiptool generate \
        --svd svd/cxd5602.svd.patched \
        --transform svd/transform.chiptool.yaml \
        --output svd-out
    just _generate_pac cxd56-pac-chiptool

# Regenerate and update svd2rust PAC
svd2rust: _svdtools
    mkdir -p svd-out
    svd2rust \
        --edition 2024 \
        --reexport-interrupt \
        -i svd/cxd5602.svd.patched \
        -o svd-out
    just _generate_pac cxd56-pac-svd2rust

_svdtools:
    svdtools patch svd/patch.yml svd/cxd5602.svd.patched
