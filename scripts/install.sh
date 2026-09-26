#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "🚀 Starting Astra Browser installation..."

# 1. Check for required dependencies
echo "🔎 Checking dependencies..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo/Rust is not installed."
    echo "Installing Cargo/Rust using pacman"
    sudo pacman -S cargo
    exit 1
fi

# Note: The repository states Firefox is a mandatory runtime dependency
if ! command -v firefox &> /dev/null; then
    echo "⚠️ Warning: Firefox was not found in your PATH."
    echo "Installing Firefox using pacman / AUR"
    sudo pacman -S firefox
fi

# Note: The REPO states Git is a necessary build dependency
if ! command -v firefox &> /dev/null; then
    echo "❌ Error: Git is not installed"
    echo "Installing Git using pacman"
    sudo pacman -S git
fi

# 2. Ensure target installation directory exists
echo "📁 Preparing installation directory..."
TARGET_DIR="$HOME/.local/bin"
mkdir -p "$TARGET_DIR"

# 3. Build the project in release mode
echo "⚙️ Building Astra Browser in release mode (this may take a minute)..."
cargo build --release

# 4. Copy the compiled binary to the destination
echo "🚚 Copying binary to $TARGET_DIR..."
cp target/release/astra-browser "$TARGET_DIR/astra-browser"

# 5. Success Message and PATH validation
echo "✅ Installation complete!"
echo "Astra Browser has been installed to: $TARGET_DIR/astra-browser"

# Check if ~/.local/bin is in the user's PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "💡 Note: '$HOME/.local/bin' is not in your system PATH."
    echo "To run 'astra-browser' from anywhere, add it to your shell profile (e.g., ~/.bashrc or ~/.zshrc):"
    echo '  export PATH="$HOME/.local/bin:$PATH"'
fi
