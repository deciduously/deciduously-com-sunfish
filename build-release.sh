#!/bin/sh
VERSION=2.0.1

cargo build --release --target=x86_64-unknown-linux-musl
rm -rf build
mkdir build
cp target/x86_64-unknown-linux-musl/release/deciduously_com build/
tar -C build/ -cJf deciduously_com_bin_"$VERSION".tar.xz .
#rm -r build