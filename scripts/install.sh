#!/bin/bash
set -e

# Install script for git-guard

echo "[git-guard] Building and installing..."

# Check for cargo
if ! command -v cargo >/dev/null 2>&1; then
  echo "[git-guard] Error: Rust and cargo are required. Install from https://rustup.rs/" >&2
  exit 1
fi

# Build and install
cargo install --path .

# Check if $HOME/.cargo/bin is in PATH
if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
  echo "[git-guard] Note: Add $HOME/.cargo/bin to your PATH to use git-guard globally."
  echo '  export PATH="$HOME/.cargo/bin:$PATH"' 
fi

echo "[git-guard] Installation complete! Run 'git-guard --version' to verify."
