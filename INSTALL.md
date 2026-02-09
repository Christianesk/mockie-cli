# 🎭 Mockie Installation Guide

Complete guide to install Mockie on Windows, macOS and Linux.

## 📋 Prerequisites

- **Rust 1.70+** - [Download](https://www.rust-lang.org/tools/install)
- **Git** (optional but recommended)

Verify you have Rust installed:
```bash
rustc --version
cargo --version
```

## 🚀 Quick Installation (Recommended)

### Windows

1. **Download the project**
   ```bash
   git clone <repo>
   cd mockie
   ```

2. **Run the installer**
   ```bash
   install.bat
   ```

   Or if you prefer Python:
   ```bash
   python install.py
   ```

3. **Open a NEW terminal and verify**
   ```bash
   mockie --version
   ```

### macOS / Linux

1. **Download the project**
   ```bash
   git clone <repo>
   cd mockie
   ```

2. **Run the installer**
   ```bash
   chmod +x install.sh
   ./install.sh
   ```

   Or if you prefer Python:
   ```bash
   python3 install.py
   ```

3. **Open a NEW terminal and verify**
   ```bash
   mockie --version
   ```

## 🔧 Manual Installation

If you prefer to do the installation manually:

### All Operating Systems

```bash
# 1. Compile in release mode
cargo build --release

# 2. The executable will be in:
# Windows: target\release\mockie.exe
# macOS/Linux: target/release/mockie
```

### Windows - Add to PATH

**Option 1: GUI**
1. Open `System Settings`
2. Search for "Environment variables"
3. Click "Edit the system environment variables"
4. Click "Environment Variables..."
5. Under "User variables", find `Path` and edit
6. Add the path where you copied `mockie.exe`
7. Click OK and restart the terminal

**Option 2: PowerShell (Admin)**
```powershell
# Copy the executable
Copy-Item "target\release\mockie.exe" "$env:PROGRAMFILES\mockie\"

# Add to PATH (requires admin)
setx PATH "$env:PATH;C:\Program Files\mockie"
```

### macOS / Linux - Add to PATH

```bash
# Copy to /usr/local/bin
sudo cp target/release/mockie /usr/local/bin/
sudo chmod +x /usr/local/bin/mockie

# Or add to your shell profile
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
# Or for zsh:
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.zshrc

# Reload configuration
source ~/.bashrc  # or ~/.zshrc
```

## 🔍 Verify Installation

After installing, open a **NEW terminal** and run:

```bash
# Show version
mockie --version

# Show help
mockie help

# View available commands
mockie serve --help
mockie add --help
mockie list --help
mockie shutdown --help
```

## 📍 Installation Locations

| System | Location |
|--------|----------|
| Windows | `C:\Program Files\mockie\mockie.exe` or `%APPDATA%\mockie\mockie.exe` |
| macOS | `/usr/local/bin/mockie` |
| Linux | `/usr/local/bin/mockie` |

## 🌐 Environment Variables

Mockie uses the following environment variables (all optional):

```bash
# Port on which the server listens (default: 3000)
export MOCKIE_PORT=8080

# Storage file (default: routes.json)
export MOCKIE_STORAGE=./data/routes.json

# Log level (default: info)
export MOCKIE_LOG_LEVEL=debug
```

Example:
```bash
MOCKIE_PORT=8080 MOCKIE_LOG_LEVEL=debug mockie serve
```

## 🚨 Common Problems

### "mockie: command not found"
- **Cause**: Mockie is not in the PATH
- **Solution**: 
  - Open a NEW terminal after installing
  - Verify that Mockie location is in PATH
  - Windows: `echo %PATH%`
  - macOS/Linux: `echo $PATH`

### "Permission denied" (macOS/Linux)
- **Cause**: Missing execution permissions
- **Solution**: 
  ```bash
  chmod +x /usr/local/bin/mockie
  ```

### "Installer requires admin" (Windows)
- **Cause**: Trying to install in Program Files without permissions
- **Solution**: 
  - Run the terminal as Administrator
  - Or install in AppData automatically

## ✨ Usage

Once installed, you can use Mockie from anywhere:

```bash
# Start server
mockie serve --port 3000

# In another terminal
mockie add --method GET --path /api/users --response '{"data":[]}'
mockie list
mockie shutdown
```

## 📚 Resources

- [Main README](README.md)
- [Technical Documentation](STRUCTURE.md)
- [Best Practices](BEST_PRACTICES.md)
- [Architecture](ARCHITECTURE.md)

## 🤝 Support

If you have problems:
1. Verify that Rust is installed correctly
2. Try reinstalling with `cargo build --release`
3. Open an issue on GitHub

---

**Happy mocking! 🎭**
