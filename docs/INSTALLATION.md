# xg Installation Guide

## System Requirements

### Minimum Requirements
- Operating System: Linux, macOS, or Windows
- Rust: 1.70.0 or higher
- Git: 2.0.0 or higher
- Memory: 50MB available RAM
- Disk Space: 10MB for installation

### Recommended
- Rust: Latest stable version
- Git: Latest version
- Terminal: A modern terminal with color support

## Installation Methods

### Method 1: Manual Installation (Recommended)

```bash
# Clone the repository
git clone https://github.com/nishujangra/xg
cd xg

# Build and install xg
cargo install --path .
```

### Method 2: From Source (Development)

```bash
# Clone the repository
git clone https://github.com/nishujangra/xg
cd xg

# Build in debug mode
cargo build

# Run directly
cargo run -- --version
# Or run the binary: ./target/debug/xg --version
```

## Verifying Installation

### Check Installation
```bash
xg --version
```

Expected output:
```
xg 0.1.0
```

### Test Basic Functionality
```bash
# Show help
xg --help

# Test project creation
xg create-go-app --help
```

## Platform-Specific Instructions

### Linux

#### Ubuntu/Debian
```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install xg
git clone https://github.com/nishujangra/xg
cd xg
cargo install --path .
```

#### CentOS/RHEL/Fedora
```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install xg
git clone https://github.com/nishujangra/xg
cd xg
cargo install --path .
```

#### Arch Linux
```bash
# Install Rust (if not installed)
sudo pacman -S rust

# Install xg
git clone https://github.com/nishujangra/xg
cd xg
cargo install --path .
```

### macOS

#### Using Homebrew
```bash
# Install Rust (if not installed)
brew install rust

# Install xg
git clone https://github.com/nishujangra/xg
cd xg
cargo install --path .
```

#### Using rustup (Recommended)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install xg
git clone https://github.com/nishujangra/xg
cd xg
cargo install --path .
```

### Windows

#### Using Windows Subsystem for Linux (WSL) - Recommended
```bash
# Follow Linux instructions above
```

#### Using PowerShell
```powershell
# Install Rust (if not installed)
# Download from https://rustup.rs/

# Install xg
git clone https://github.com/nishujangra/xg
cd xg
cargo install --path .
```

## PATH Configuration

### Manual PATH Setup

You may need to add `$HOME/.cargo/bin` to your PATH if xg is not found after installation.

#### Bash/Zsh
Add to `~/.bashrc` or `~/.zshrc`:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then reload:
```bash
source ~/.bashrc  # or ~/.zshrc
```

#### Fish Shell
Add to `~/.config/fish/config.fish`:
```fish
set -gx PATH $HOME/.cargo/bin $PATH
```

#### Windows PowerShell
Add to your PowerShell profile:
```powershell
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

## Testing the Installation

### Basic Test
```bash
# Test version
xg --version

# Test help
xg --help
```

### Functionality Test
```bash
# Test project creation
xg create-go-app --project "test-project" --rest-framework gin
cd test-project
ls -la  # Should show generated files

# Test git wrapper (if in a git repo)
git status
git add .
git commit -m "Initial commit"
```

### Integration Test
```bash
# In a real git repository
git push origin main

# Should either:
# - Push successfully (if clean)
# - Block push (if unwanted files)
# - Show HTTPS warning (if using HTTPS)
```

## Troubleshooting

### Common Issues

#### "cargo: command not found"
Problem: Rust is not installed or not in PATH.

Solution:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### "xg: command not found"
Problem: xg is installed but not in PATH.

Solution:
```bash
# Add cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Verify installation
ls -la ~/.cargo/bin/xg
```

#### "Permission denied" during installation
Problem: Insufficient permissions to write to cargo directory.

Solution:
```bash
# Fix cargo directory permissions
sudo chown -R $USER:$USER ~/.cargo

# Or install with user permissions
cargo install --path . --user
```

#### "Failed to compile" errors
Problem: Missing system dependencies or incompatible Rust version.

Solution:
```bash
# Update Rust
rustup update

# Install build dependencies (Ubuntu/Debian)
sudo apt-get install build-essential

# Install build dependencies (CentOS/RHEL)
sudo yum groupinstall "Development Tools"
```

#### "git: command not found"
Problem: Git is not installed.

Solution:
```bash
# Ubuntu/Debian
sudo apt-get install git

# CentOS/RHEL
sudo yum install git

# macOS
brew install git

# Windows
# Download from https://git-scm.com/
```

### Getting Help

#### Check Installation Status
```bash
# Check Rust version
rustc --version

# Check cargo version
cargo --version

# Check git version
git --version

# Check xg installation
which xg
xg --version
```

#### Debug Installation
```bash
# Verbose cargo install
cargo install --path . --verbose

# Check cargo environment
cargo env

# List installed binaries
ls -la ~/.cargo/bin/
```

## Updating xg

### Update from Source
```bash
# Pull latest changes
cd xg
git pull origin main

# Reinstall
cargo install --path . --force
```

### Check for Updates
```bash
# Check current version
xg --version

# Check repository for updates
cd xg
git fetch origin
git status
```

## Uninstalling xg

### Remove Binary
```bash
# Remove the binary
rm ~/.cargo/bin/xg

# Or uninstall via cargo
cargo uninstall xg
```

### Clean Up Repository
```bash
# Remove local repository
rm -rf xg/
```

## Next Steps

After installation:

1. Read the Usage Guide to learn how to use xg
2. Test with your repositories to see it in action
3. Set up git aliases for convenience
4. Share with your team to prevent accidental commits