#!/bin/bash

set -e

OS="$(uname -s)"
ARCH="$(uname -m)"

echo $OS
echo $ARCH

BINARY_URL=""
BINARY_FILE="n"


if [ "$OS" = "Darwin" ]; then
  if [ "$ARCH" = "x86_64" ]; then
    BINARY_URL="https://github.com/sheinsight/ni/releases/download/v0.0.4/x86_64-apple-darwin.tar.gz"
    INSTALL_DIR="/usr/local/bin"
  elif [ "$ARCH" = "arm64" ]; then
    BINARY_URL="https://github.com/sheinsight/ni/releases/download/v0.0.4/aarch64-apple-darwin.tar.gz"
    INSTALL_DIR="$HOME/.local/bin"
  fi
elif [ "$OS" = "Linux" ]; then
  BINARY_URL="https://github.com/sheinsight/ni/releases/download/v0.0.4/x86_64-unknown-linux-musl.tar.gz"
  INSTALL_DIR="$HOME/.local/bin"
elif [ "$OS" = "MINGW*" ]; then
  BINARY_URL="https://github.com/sheinsight/ni/releases/download/v0.0.4/x86_64-pc-windows-gnu.tar.gz"
  INSTALL_DIR="$HOME/.local/bin"
else
  echo "Your OS is not supported." >&2
  exit 1
fi

if [ -d "$HOME/.local" ]; then
  INSTALL_DIR="$HOME/.local/bin"
elif [ -n "$XDG_DATA_HOME" ]; then
  INSTALL_DIR="$XDG_DATA_HOME"
elif [ "$OS" = "Darwin" ]; then
  INSTALL_DIR="/usr/local/bin"
else
  INSTALL_DIR="$HOME/.local/bin"
fi


# 如果目录不存在则创建
mkdir -p $INSTALL_DIR

# 将原文件下载到临时文件
temp=$(mktemp)
echo $BINARY_URL

curl -L $BINARY_URL -o $temp

# 解压到目标目录
tar -zxvf $temp -C $INSTALL_DIR



# 下载二进制文件
# curl -L $BINARY_URL -o "$INSTALL_DIR/$BINARY_FILE"

# 将文件设置为可执行
chmod +x "$INSTALL_DIR/$BINARY_FILE"

echo "Installation completed"

# 检查是否安装成功
if command -v $INSTALL_DIR/$BINARY_FILE >/dev/null 2>&1; then
  echo "Successfully installed $BINARY_FILE"
else
  echo "Installation of $BINARY_FILE failed" >&2
  exit 1
fi

# 找出所使用的 shell
CURRENT_SHELL=$(basename "$SHELL")

# 根据 shell 设置 profile 文件的文件名
if [[ "$CURRENT_SHELL" == "bash" ]]; then
  # bash 的 profile 文件名
  SHELL_PROFILE="$HOME/.bashrc"
elif [[ "$CURRENT_SHELL" == "zsh" ]]; then
  # zsh 的 profile 文件名
  SHELL_PROFILE="${ZDOTDIR:-$HOME}/.zshrc"
else
  # 其他 shell 此处无法处理，需要用户自行设定
  echo "Unrecognized shell $CURRENT_SHELL, you might need to add the directory to your PATH manually." >&2
  exit 1
fi

# 添加到环境变量
{
  echo ""
  echo '# Add ni to PATH'
  echo "export PATH=$INSTALL_DIR:\$PATH"
} >> "$SHELL_PROFILE"

echo "Successfully modified $SHELL_PROFILE"
echo "Please start a new terminal session for the changes to take effect, or source the profile file directly using:"
echo "source $SHELL_PROFILE"