#!/bin/sh
set -e

REPO="yazaldefilimone/lemon"
BIN="lemon"
INSTALL_DIR="/usr/local/bin"

OS=$(uname -s)
ARCH=$(uname -m)

case $OS in
    "Linux") PLATFORM="linux" ;;
    "Darwin") PLATFORM="macos" ;;
    *) echo "error: unsupported os: $OS" && exit 1 ;;
esac

case $ARCH in
    "x86_64") ARCH="x86_64" ;;
    "arm64") ARCH="aarch64" ;;
    *) echo "error: unsupported arch: $ARCH" && exit 1 ;;
esac

BIN_URL="https://github.com/$REPO/releases/latest/download/$BIN-$PLATFORM-$ARCH"

if ! command -v curl >/dev/null 2>&1; then
    echo "error: 'curl' is required. Install it and try again."
    exit 1
fi

if [ ! -w "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    echo "Installing to $INSTALL_DIR. Add it to PATH if needed."
fi

echo "Downloading Lemon ($PLATFORM-$ARCH)..."
curl -fsSL "$BIN_URL" -o "$BIN"
chmod +x "$BIN"
mv "$BIN" "$INSTALL_DIR"

echo "Installed! Run '$BIN --help' to get started."
