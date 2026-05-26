default:
    @just --list

chiptool: svdtools
    mkdir -p svd-out
    chiptool generate \
        --svd svd/cxd5602.svd.patched \
        --transform svd/transform.chiptool.yaml \
        --output svd-out
    rustfmt svd-out/lib.rs
    sed -i '/#!\[no_std]/d' svd-out/lib.rs
    form -f -i svd-out/lib.rs -o svd-out/src/
    rm -rf cxd56-pac-chiptool/src
    cp -r svd-out/src/ cxd56-pac-chiptool/
    rm -rf svd-out
    cargo fmt -p cxd56-pac-chiptool

svd2rust: svdtools
    mkdir -p svd-out
    svd2rust -i svd/cxd5602.svd.patched -o svd-out
    form -f -i svd-out/lib.rs -o svd-out/src/
    rm -rf cxd56-pac-svd2rust/src
    cp -r svd-out/src/ cxd56-pac-svd2rust/
    rm -rf svd-out
    cargo fmt -p cxd56-pac-svd2rust

svdtools:
    svdtools patch svd/patch.yml svd/cxd5602.svd.patched
