#!/usr/bin/env bash

if [[ "$TRAVIS_OS_NAME" == "linux" ]] || [[ "$OSTYPE" == "linux-gnu" ]]; then
    docker run --rm --privileged multiarch/qemu-user-static:register --reset
    docker run -t -v $(pwd):/usr/src jeikabu/debian-rust:arm32v7-stretch-1.33.0 /bin/bash -c "cd /usr/src; cargo test"
fi