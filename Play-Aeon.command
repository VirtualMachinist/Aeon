#!/bin/zsh
# macOS playtest launcher; build artifacts stay outside the source/vault.
set -eu
cd "${0:A:h}"
source "$HOME/.cargo/env"
export CARGO_TARGET_DIR="$HOME/Library/Caches/AeonBuild"
# Pack the sprite pages once (a few seconds); the game then starts fast
# and small. Delete crates/client/assets/packed to force a repack.
if [[ ! -f crates/client/assets/packed/raya.manifest ]]; then
  cargo run --release --offline -p aeon -- --pack
fi
exec cargo run --release --offline -p aeon -- "$@"
