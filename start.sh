#!/bin/bash
_exit() {
    echo "Usage: ./start.sh [dev|prod]"
    exit 1
}

if [ "$#" -ne 1 ]; then
    _exit
fi

if [ "$1" = "dev" ]; then
    RUST_LOG=debug cargo run
elif [ "$1" = "prod" ]; then
    cargo build -r
else
    _exit
fi

