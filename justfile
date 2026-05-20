default:
    @just --list

chiptool:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir out
    cd out
    chiptool generate --svd ../cxd5602.svd.patched --transform ../transform.chiptool.yaml
    rustfmt lib.rs
    sed -i '/#!\[no_std]/d' lib.rs
    form -i lib.rs -o src/
    cp -r src/ ../cxd56-pac-chiptool/