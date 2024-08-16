#!/bin/sh
VERSION=2.0.4

cargo build --release --target=x86_64-unknown-linux-musl
rm -rf build
mkdir build
cp target/x86_64-unknown-linux-musl/release/deciduously_com_sunfish build/
tar -C build/ -cJf deciduously_com_sunfish_bin_"$VERSION".tar.xz .
rm -rf build