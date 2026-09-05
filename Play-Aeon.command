#!/bin/zsh
# macOS playtest launcher; build artifacts stay outside the source/vault.
set -eu
cd "${0:A:h}"
source "$HOME/.cargo/env"
export CARGO_TARGET_DIR="$HOME/Library/Caches/AeonBuild"
exec cargo run --release --offline -p aeon -- "$@"
