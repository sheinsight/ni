#!/bin/bash

# rustup target list
# rustup target add x86_64-unknown-linux-musl
# rustup target add x86_64-pc-windows-gnu


if [[ $(echo $0 | awk '/^\//') == $0 ]]; then
    ABSPATH=$(dirname $0)
else
    ABSPATH=$PWD/$(dirname $0)
fi
cd ${ABSPATH}

cross build --release --target x86_64-unknown-linux-musl 
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-apple-darwin

rm -rf ./target/gz 
mkdir ./target/gz
tar -czvf ./target/gz/x86_64-unknown-linux-musl.tar.gz -C ./target/x86_64-unknown-linux-musl/release n
tar -czvf ./target/gz/x86_64-pc-windows-gnu.tar.gz -C ./target/x86_64-pc-windows-gnu/release n.exe
tar -czvf ./target/gz/aarch64-apple-darwin.tar.gz -C ./target/aarch64-apple-darwin/release n
tar -czvf ./target/gz/x86_64-apple-darwin.tar.gz -C ./target/x86_64-apple-darwin/release n