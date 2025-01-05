#!/bin/bash

set -e  # exit on errors
set -u  # error on unset variables

# detect os and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')  # normalize os to lowercase
ARCH=$(uname -m)

echo "os: $OS, arch: $ARCH"

STATUS=0  # 0 = success, 1 = failure

ask() {
  echo "$1 is missing. oh no, you need it... let me help you? [y/n]"
  read -r answer
  case "$answer" in
    [yY][eE][sS]|[yY])
      return 0
      ;;
    *)
      echo "skipping $1 installation."
      return 1
      ;;
  esac
}

manual_install() {
  echo "please install $1 manually:"
  echo "  $2"
  STATUS=1  # mark as failed
}

# rust
install_rust() {
  if command -v rustc &> /dev/null; then
    echo "rust is already installed."
  else
    ask "rust" && {
      echo "installing rust..."
      if curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; then
        export PATH="$HOME/.cargo/bin:$PATH"
        echo "rust installed successfully."
      else
        echo "failed to install rust."
        STATUS=1
      fi
    } || manual_install "rust" "https://rustup.rs/"
  fi
}

# llvm
install_llvm() {
  if command -v llvm-config &> /dev/null; then
    echo "llvm is already installed."
  else
    ask "llvm" && {
      echo "installing llvm..."
      case "$OS" in
        linux)
          if command -v apt &> /dev/null; then
            sudo apt update -qq
            if sudo apt install -y llvm clang; then
              echo "llvm installed successfully."
            else
              echo "failed to install llvm."
              STATUS=1
            fi
          elif command -v pacman &> /dev/null; then
            if sudo pacman -Syu --noconfirm llvm clang; then
              echo "llvm installed successfully."
            else
              echo "failed to install llvm."
              STATUS=1
            fi
          else
            echo "unsupported package manager for linux."
            STATUS=1
          fi
          ;;
        darwin)
          if brew install llvm; then
            echo "llvm installed successfully."
          else
            echo "failed to install llvm."
            STATUS=1
          fi
          ;;
        windows)
          manual_install "llvm" "https://llvm.org/"
          ;;
        *)
          echo "unsupported os for llvm installation."
          STATUS=1
          ;;
      esac
    } || manual_install "llvm" "https://llvm.org/"
  fi
}

# mold
install_mold() {
  if command -v mold &> /dev/null; then
    echo "mold is already installed."
  else
    ask "mold" && {
      echo "installing mold..."
      case "$OS" in
        linux)
          if command -v apt &> /dev/null; then
            if sudo apt update -qq && sudo apt install -y mold; then
              echo "mold installed successfully."
            else
              echo "failed to install mold via apt."
              STATUS=1
            fi
          elif command -v pacman &> /dev/null; then
            if sudo pacman -Syu --noconfirm mold; then
              echo "mold installed successfully."
            else
              echo "failed to install mold via pacman."
              STATUS=1
            fi
          else
            echo "unsupported package manager for linux."
            STATUS=1
          fi
          ;;
        darwin)
            echo "we use ld instead of mold on macOS."
          fi
          ;;
        windows)
          echo "mold is not supported on windows."
          echo "but don't worry I'll try to use an alternative (lld), i hope lld-link is installed along with llvm."
          ;;
        *)
          echo "unsupported os for mold installation."
          STATUS=1
          ;;
      esac
    } || manual_install "mold" "https://github.com/rui314/mold"
  fi
}

# run
case "$OS" in
  linux|darwin)
    install_rust
    install_llvm
    install_mold
    ;;
  windows)
    echo "windows detected."
    install_rust
    install_llvm
    install_mold
    ;;
  *)
    echo "unsupported system: $OS on $ARCH"
    echo "please install the required dependencies manually:"
    echo "  - rust: https://rustup.rs/"
    echo "  - llvm: https://llvm.org/"
    echo "  - mold: https://github.com/rui314/mold"
    STATUS=1
    ;;
esac

if [ "$STATUS" -eq 0 ]; then
  echo "successfully..."
else
  echo "failed. please install missing dependencies manually."
  exit 1
fi
