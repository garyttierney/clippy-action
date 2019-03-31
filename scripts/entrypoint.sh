#!/bin/sh

set -e
cargo clippy --message-format json > clippy.out.json
clippy-action