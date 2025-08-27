#!/bin/bash

echo "Uninstalling AUI Next.js Generator..."
echo

# Remove executable
INSTALL_DIR="$HOME/.local/bin"
BINARY_PATH="$INSTALL_DIR/aui-next-gen"

if [ -f "$BINARY_PATH" ]; then
    echo "Removing executable from $INSTALL_DIR..."
    rm "$BINARY_PATH"
    echo "✅ Executable removed"
else
    echo "⚠️  Executable not found at $BINARY_PATH"
fi

# Remove from PATH in shell config files
echo "Removing from PATH..."

for CONFIG_FILE in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
    if [ -f "$CONFIG_FILE" ]; then
        # Create backup
        cp "$CONFIG_FILE" "$CONFIG_FILE.backup"
        
        # Remove the PATH line
        grep -v "export PATH.*$INSTALL_DIR" "$CONFIG_FILE" > "$CONFIG_FILE.tmp" && mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"
        echo "Updated $CONFIG_FILE"
    fi
done

echo
echo "✅ Uninstallation completed!"
echo
echo "Notes:"
echo "- Shell config backups created with .backup extension"
echo "- Please restart your terminal or run 'source ~/.bashrc' (or ~/.zshrc)"
echo "- If installed via 'cargo install', also run: cargo uninstall aui-next-generator"
echo