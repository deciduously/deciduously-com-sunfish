#!/bin/sh
set -eu
tag=2.0.2
#cargo build --target x86_64-unknown-linux-musl
docker build -t deciduously-com:"$tag" .
