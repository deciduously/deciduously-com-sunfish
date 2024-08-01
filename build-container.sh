#!/bin/sh
set -eu
tag=2.0.3
docker build -t deciduously-com-sunfish:"$tag" .
