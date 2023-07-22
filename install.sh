#!/bin/bash

# 变量设置
BINARY_URL="https://github.com/sheinsight/ni/releases/download/v0.0.1/n"
BINARY_FILE="n"
INSTALL_DIR="$HOME/.local/bin"
SHELL_PROFILE="$HOME/.bashrc" # 默认为 Bash

# 根据不同的 Shell 选择配置文件
if [ $SHELL == "/bin/zsh" ]; then
    SHELL_PROFILE="$HOME/.zshrc"
elif [ $SHELL == "/bin/bash" ]; then
    SHELL_PROFILE="$HOME/.bashrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_PROFILE="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_PROFILE="$HOME/.zshrc"
fi

# 如果目录不存在则创造
mkdir -p $INSTALL_DIR

# 下载二进制文件
curl -o $BINARY_FILE $BINARY_URL

# 将文件设置为可执行
chmod +x ./$BINARY_FILE

# 移动文件到安装目录
mv ./$BINARY_FILE $INSTALL_DIR

echo "Installation completed"

# 检查是否安装成功
if command -v $INSTALL_DIR/$BINARY_FILE >/dev/null 2>&1; then
    echo "Successfully installed $BINARY_FILE"
else
    echo "Installation of $BINARY_FILE failed"
    exit 1
fi

# 添加到环境变量
echo 'export PATH=$PATH:'$INSTALL_DIR >> $SHELL_PROFILE

# 使改变立即生效
source $SHELL_PROFILE

echo "The $BINARY_FILE path has been added to the system PATH through $SHELL_PROFILE"