#!/usr/bin/env python3
"""
Mockie Installer - Cross-platform (Windows, macOS, Linux)
Installs Mockie and automatically configures environment variables
"""

import os
import sys
import subprocess
import platform
import shutil
from pathlib import Path


def print_header(text):
    print("\n" + "=" * 60)
    print(f"  {text}")
    print("=" * 60 + "\n")


def print_success(text):
    print(f"✅ {text}")


def print_error(text):
    print(f"❌ {text}")
    sys.exit(1)


def print_info(text):
    print(f"ℹ️  {text}")


def get_os():
    """Detects the operating system"""
    system = platform.system()
    if system == "Windows":
        return "windows"
    elif system == "Darwin":
        return "macos"
    elif system == "Linux":
        return "linux"
    else:
        print_error(f"Unsupported operating system: {system}")


def run_command(cmd, description):
    """Executes a command and handles errors"""
    print_info(description)
    try:
        result = subprocess.run(cmd, shell=True, check=True, capture_output=True, text=True)
        return True
    except subprocess.CalledProcessError as e:
        print_error(f"Error al ejecutar: {cmd}\n{e.stderr}")


def build_release():
    """Compiles the project in release mode"""
    print_header("🔨 Compiling Mockie in release mode...")
    
    if not run_command("cargo build --release", "Compiling..."):
        print_error("Compilation failed")
    
    print_success("Compilation completed")


def get_install_path(os_type):
    """Returns the installation path based on the OS"""
    if os_type == "windows":
        # C:\Program Files\mockie or AppData\Local
        program_files = os.getenv("PROGRAMFILES", "C:\\Program Files")
        return os.path.join(program_files, "mockie")
    else:
        # /usr/local/bin for macOS/Linux
        return "/usr/local/bin"


def install_windows():
    """Installation on Windows"""
    print_header("🪟 Installation on Windows")
    
    source = "target\\release\\mockie.exe"
    if not os.path.exists(source):
        print_error(f"Executable not found: {source}")
    
    # Create folder in Program Files
    install_path = get_install_path("windows")
    try:
        os.makedirs(install_path, exist_ok=True)
        dest = os.path.join(install_path, "mockie.exe")
        shutil.copy2(source, dest)
        print_success(f"Installed in: {dest}")
    except PermissionError:
        # If no permissions, install in AppData
        appdata = os.getenv("APPDATA")
        install_path = os.path.join(appdata, "mockie")
        os.makedirs(install_path, exist_ok=True)
        dest = os.path.join(install_path, "mockie.exe")
        shutil.copy2(source, dest)
        print_success(f"Installed in: {dest}")
    
    # Add to PATH
    print_info("Adding to PATH...")
    add_to_path_windows(install_path)
    
    print_success("Mockie installed on Windows")
    print_info(f"You can use: mockie serve")


def install_unix(os_type):
    """Installation on macOS/Linux"""
    os_name = "macOS" if os_type == "macos" else "Linux"
    print_header(f"🐧 Installation on {os_name}")
    
    source = "target/release/mockie"
    if not os.path.exists(source):
        print_error(f"Executable not found: {source}")
    
    # Copy to /usr/local/bin
    install_path = get_install_path(os_type)
    dest = os.path.join(install_path, "mockie")
    
    print_info(f"Copying to {dest}...")
    try:
        # Try without sudo first
        if os.access(install_path, os.W_OK):
            shutil.copy2(source, dest)
        else:
            # Needs sudo
            run_command(f"sudo cp {source} {dest}", f"Copying to {dest} (requires sudo)...")
        
        # Give execution permissions
        run_command(f"chmod +x {dest}", "Configuring permissions...")
        print_success(f"Installed in: {dest}")
    except Exception as e:
        print_error(f"Error during installation: {e}")
    
    # Add to PATH
    add_to_path_unix(os_type)
    
    print_success(f"Mockie installed on {os_name}")
    print_info(f"You can use: mockie serve")


def add_to_path_windows(path):
    """Adds the path to PATH on Windows"""
    print_info("Updating PATH on Windows...")
    try:
        import winreg
        
        key = winreg.OpenKey(
            winreg.HKEY_CURRENT_USER,
            r"Environment",
            0,
            winreg.KEY_ALL_ACCESS
        )
        
        current_path = winreg.QueryValueEx(key, "Path")[0]
        
        if path not in current_path:
            new_path = f"{current_path};{path}"
            winreg.SetValueEx(key, "Path", 0, winreg.REG_EXPAND_SZ, new_path)
            print_success("PATH updated")
            print_info("⚠️  Open a new terminal for changes to take effect")
        else:
            print_info("Already in PATH")
        
        winreg.CloseKey(key)
    except Exception as e:
        print_error(f"Error updating PATH: {e}")


def add_to_path_unix(os_type):
    """Adds the path to PATH on macOS/Linux"""
    print_info("Updating PATH in shell...")
    
    home = str(Path.home())
    shell_files = []
    
    if os_type == "macos":
        shell_files = [
            os.path.join(home, ".zshrc"),
            os.path.join(home, ".bash_profile"),
        ]
    else:  # Linux
        shell_files = [
            os.path.join(home, ".bashrc"),
            os.path.join(home, ".bash_profile"),
            os.path.join(home, ".zshrc"),
        ]
    
    path_line = "\nexport PATH=\"/usr/local/bin:$PATH\""
    
    for shell_file in shell_files:
        if os.path.exists(shell_file):
            with open(shell_file, "r") as f:
                content = f.read()
            
            if "/usr/local/bin" not in content:
                with open(shell_file, "a") as f:
                    f.write(path_line)
                print_success(f"Added to: {shell_file}")


def verify_installation(os_type):
    """Verifies that the installation was successful"""
    print_header("🔍 Verifying installation...")
    
    try:
        if os_type == "windows":
            result = subprocess.run("mockie --version", shell=True, capture_output=True, text=True)
        else:
            result = subprocess.run("mockie --version", shell=True, capture_output=True, text=True)
        
        if result.returncode == 0:
            print_success("Mockie is correctly installed")
            print_info(f"Version: {result.stdout.strip()}")
            return True
        else:
            print_info("⚠️  Could not verify. Open a new terminal and try: mockie --version")
            return False
    except Exception as e:
        print_info("⚠️  Could not verify: Open a new terminal and try: mockie --version")
        return False


def main():
    """Main function"""
    print_header("🎭 MOCKIE CROSS-PLATFORM INSTALLER")
    
    os_type = get_os()
    print_success(f"Operating system detected: {os_type.upper()}")
    
    # Compile in release
    build_release()
    
    # Install based on OS
    if os_type == "windows":
        install_windows()
    elif os_type == "macos":
        install_unix("macos")
    elif os_type == "linux":
        install_unix("linux")
    
    # Verify
    verify_installation(os_type)
    
    print_header("✨ INSTALLATION COMPLETED!")
    print_info("Next steps:")
    print_info("1. Open a new terminal")
    print_info("2. Run: mockie --help")
    print_info("3. Start using Mockie!")


if __name__ == "__main__":
    main()
