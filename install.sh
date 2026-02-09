#!/bin/bash

# Mockie installer for macOS/Linux
# Compiles and installs Mockie, automatically adding it to PATH

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macOS"
    SHELL_PROFILE="$HOME/.zshrc"
    [ -f "$HOME/.bash_profile" ] && SHELL_PROFILE="$HOME/.bash_profile"
else
    OS="Linux"
    SHELL_PROFILE="$HOME/.bashrc"
    [ -f "$HOME/.zshrc" ] && SHELL_PROFILE="$HOME/.zshrc"
fi

echo ""
echo "============================================================"
echo "  🎭 MOCKIE CROSS-PLATFORM INSTALLER - $OS"
echo "============================================================"
echo ""

# Compile in release
echo -e "${BLUE}🔨 Compiling Mockie in release mode...${NC}"
if ! cargo build --release; then
    echo -e "${RED}❌ Error: Compilation failed${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Compilation completed${NC}"
echo ""

# Define paths
SOURCE="target/release/mockie"
INSTALL_PATH="/usr/local/bin"

# Verify that the executable exists
if [ ! -f "$SOURCE" ]; then
    echo -e "${RED}❌ Error: Executable not found at $SOURCE${NC}"
    exit 1
fi

# Copy to /usr/local/bin
echo -e "${BLUE}ℹ️  Copying to $INSTALL_PATH...${NC}"

# Check if we can write to /usr/local/bin without sudo
if [ -w "$INSTALL_PATH" ]; then
    cp "$SOURCE" "$INSTALL_PATH/mockie"
    chmod +x "$INSTALL_PATH/mockie"
    echo -e "${GREEN}✅ Installed in: $INSTALL_PATH/mockie${NC}"
else
    # Needs sudo
    echo -e "${BLUE}ℹ️  Sudo permissions required...${NC}"
    sudo cp "$SOURCE" "$INSTALL_PATH/mockie"
    sudo chmod +x "$INSTALL_PATH/mockie"
    echo -e "${GREEN}✅ Installed in: $INSTALL_PATH/mockie${NC}"
fi

# Add to PATH
echo ""
echo -e "${BLUE}ℹ️  Updating PATH...${NC}"

# Check if it's already in the shell profile
if grep -q "export PATH=\"/usr/local/bin:\$PATH\"" "$SHELL_PROFILE" 2>/dev/null || \
   grep -q "export PATH=/usr/local/bin:\$PATH" "$SHELL_PROFILE" 2>/dev/null; then
    echo -e "${BLUE}ℹ️  Already in PATH${NC}"
else
    # Add to shell profile
    echo "" >> "$SHELL_PROFILE"
    echo "# Mockie installation" >> "$SHELL_PROFILE"
    echo 'export PATH="/usr/local/bin:$PATH"' >> "$SHELL_PROFILE"
    echo -e "${GREEN}✅ Added to: $SHELL_PROFILE${NC}"
fi

# Load the new PATH in the current terminal
source "$SHELL_PROFILE" 2>/dev/null || true

echo ""
echo "============================================================"
echo "✨ INSTALLATION COMPLETED!"
echo "============================================================"
echo ""
echo "Next steps:"
echo " 1. If this is your first time, open a new terminal"
echo " 2. Run: mockie --help"
echo " 3. Start using Mockie!"
echo ""
