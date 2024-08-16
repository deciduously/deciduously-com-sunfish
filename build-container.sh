#!/bin/sh
set -eu
tag=2.0.4
docker buildx build --push --platform linux/amd64 -t deciduously0/deciduously-com-sunfish:"$tag" .
