#!/bin/bash

echo "Installing AUI Next.js Generator..."
echo

# Build the project in release mode
echo "Building project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Error: Failed to build the project"
    exit 1
fi

# Create install directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy executable
echo "Copying executable to $INSTALL_DIR..."
cp target/release/aui-next-gen "$INSTALL_DIR/"
if [ $? -ne 0 ]; then
    echo "Error: Failed to copy executable"
    exit 1
fi

# Make executable
chmod +x "$INSTALL_DIR/aui-next-gen"

# Check if directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Adding $INSTALL_DIR to PATH..."
    
    # Detect shell and add to appropriate config file
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
    else
        SHELL_CONFIG="$HOME/.profile"
    fi
    
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
    echo "Added to $SHELL_CONFIG"
    echo "Please run: source $SHELL_CONFIG"
    echo "Or restart your terminal"
else
    echo "Directory already in PATH"
fi

echo
echo "✅ Installation completed successfully!"
echo
echo "To use the tool, run:"
echo "   aui-next-gen my-project-name"
echo
echo "Or with options:"
echo "   aui-next-gen --skip-install my-project"
echo "   aui-next-gen --help"
echo