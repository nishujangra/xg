# 📖 xg Complete Usage Guide

## 🚀 Quick Installation

### Prerequisites
- **Rust** (install from [rustup.rs](https://rustup.rs/))
- **Git** (should already be installed)

### Install xg
```bash
# Clone the repository
git clone https://github.com/nishujangra/git-guard.git
cd git-guard

# Install manually
cargo install --path .
```

### Make xg Available Globally

If you get "command not found" after installation, add Cargo's bin directory to your PATH:

```bash
# Add to your shell config (~/.bashrc, ~/.zshrc, etc.)
export PATH="$HOME/.cargo/bin:$PATH"

# Reload your shell
source ~/.bashrc  # or ~/.zshrc

# Now you can use xg from anywhere
xg --version
```

### Verify Installation
```bash
xg --version
# Should show: xg 0.1.0
```

### Optional: Replace Git Globally
```bash
# Make xg your default git command
echo 'alias git="xg"' >> ~/.bashrc
source ~/.bashrc

# Now 'git' commands use xg automatically!
git --version  # Shows xg version
```

---

## 📋 Basic Usage

### Two Ways to Use xg

#### 1. 🎯 Project Creation (New Projects)
```bash
# Create a new Go API server
xg init -lang golang -name "my-api"
# Interactive: Choose framework (echo/gin/fiber)

# Create a React application
xg init -lang javascript -name "my-app"

# Create a Rust CLI tool
xg init -lang rust -name "my-tool"
```

#### 2. 🔄 Git Wrapper (Existing Projects)
```bash
# Option A: Replace git globally (recommended)
echo 'alias git="xg"' >> ~/.bashrc
source ~/.bashrc

# Now use git normally with enhanced safety
git status              # Enhanced status
git add .               # Normal staging
git commit -m "feat: add new feature"  # Normal commits
git push origin main    # Safe push with file blocking

# Option B: Use xg directly
xg status
xg add .
xg commit -m "feat: add new feature"
xg push origin main
```

---

## 🚀 Project Creation Commands

### xg init
Create new projects with intelligent templating.

**Syntax:**
```bash
xg init -l <language> -n <project-name> [options]
```

**Examples:**
```bash
# Interactive framework selection
xg init -lang golang -name "api-server"

# Pre-select framework
xg init -lang javascript -name "dashboard" -f react

# Skip interactive prompts
xg init -lang rust -name "cli-tool" --no-git
```

**Supported Languages:**
- `golang` - Go (echo, gin, fiber, chi, stdlib)
- `javascript` - JavaScript (react, vue, svelte, node)
- `typescript` - TypeScript (react, vue, node, nestjs)
- `rust` - Rust (cli, lib, web-app, api)
- `python` - Python (fastapi, flask, django, data-science)

---

## 🔄 Git Wrapper Commands

xg supports all standard git commands with enhanced safety features.

### Core Git Commands
xg supports all standard git commands transparently:

**Repository Operations:**
```bash
xg init                    # Create repository
xg clone <url>            # Clone repository
xg status                  # Show working tree status
xg log                     # Show commit history
xg diff                    # Show changes
```

**Staging & Committing:**
```bash
xg add <files>            # Stage files
xg add .                  # Stage all changes
xg commit -m "message"    # Commit staged changes
xg commit -am "message"   # Add & commit all changes
```

**Branching & Merging:**
```bash
xg branch                  # List branches
xg branch <name>          # Create branch
xg checkout <branch>      # Switch branch
xg merge <branch>         # Merge branch
xg rebase <branch>        # Rebase branch
```

**Remote Operations:**
```bash
xg remote -v              # List remotes
xg fetch                  # Fetch from remote
xg pull                   # Pull changes
xg push origin main       # SAFE push with file blocking
xg push origin main --force  # Force push (with checks)
```

### Enhanced Safety Features

**Safe Push Command:**
```bash
xg push <remote> <branch> [options]
```

**Examples:**
```bash
# Basic safe push
xg push origin main

# Force push (use with caution)
xg push origin main --force

# Push with tags
xg push origin main --tags

# Push specific branch
xg push origin feature-branch
```

---

## 🛡️ Safety Features

### Enhanced Git Operations
When using xg as a git wrapper, it provides additional safety and intelligence:

### 1. **Smart File Blocking (Push Protection)**
- **Automatic Detection**: Scans staged files before push
- **Comprehensive Blocking**: Prevents unwanted files (`.env`, `node_modules/`, build artifacts)
- **Clear Feedback**: Shows exactly which files are blocked and why
- **Recovery Guidance**: Provides commands to fix the issue

### 2. **Security Enhancements**
- **HTTPS Warnings**: Recommends SSH for secure authentication
- **Remote Validation**: Checks remote configurations
- **Credential Safety**: Never stores or exposes credentials

### 3. **Intelligent Status Display**
- **Enhanced Output**: More informative than standard git status
- **Safety Indicators**: Shows potential issues before they become problems
- **Suggestion System**: Provides helpful next steps

### 4. **Transparent Git Compatibility**
- **Full Command Support**: All git commands work identically
- **Zero Breaking Changes**: Existing git workflows continue to work
- **Performance**: Minimal overhead (typically <200ms)

---

## 📊 Example Outputs

### 🎯 Project Creation Example
```bash
$ xg init -lang golang -name "api-server"

🚀 Creating new Go project: api-server
📁 Directory: ./api-server/

🎯 Which framework would you like to use for your Go API server?

1. Echo   - High performance, minimalist Go web framework
2. Gin    - The fastest full-featured web framework for Go
3. Fiber  - Express-inspired web framework built on Fasthttp
4. Chi    - Lightweight, idiomatic and composable router

Enter your choice (1-4) [default: 2]: 2

✅ Selected: Gin
🔧 Generating project structure...
📦 Creating go.mod, main.go, handlers, middleware...
🐳 Adding Dockerfile and docker-compose.yml...
📝 Setting up README.md and .gitignore...
🔐 Creating .env.example...
🌱 Initializing git repository...
🎉 Project 'api-server' created successfully!

Next steps:
  cd api-server
  go mod tidy
  go run main.go
```

### ✅ Successful Push (Clean Repository)
```bash
$ xg push origin main
🚀 xg: Checking staged files before push...
📤 Target: origin -> main
📋 Found 0 staged files
✅ All checks passed! Executing git push...
🚀 Successfully pushed to origin:main
```

### ✅ Successful Push (With Staged Files)
```bash
$ xg push origin main
🚀 xg: Checking staged files before push...
📤 Target: origin -> main
📋 Found 2 staged files
📁 Staged files:
   - src/main.rs
   - README.md
✅ All checks passed! Executing git push...
🚀 Successfully pushed to origin:main
```

### 🚫 Blocked Push (Unwanted Files)
```bash
$ xg push origin main
🚀 xg: Checking staged files before push...
📤 Target: origin -> main
📋 Found 3 staged files
📁 Staged files:
   - src/main.rs
   - .env
   - node_modules/lodash/index.js

🚫 Push blocked! Found 2 blocked file(s):
   ❌ .env (blocked by pattern: .env)
   ❌ node_modules/lodash/index.js (blocked by pattern: node_modules/)

💡 To fix this:
   1. Remove blocked files: xg rm --cached <file>
   2. Add to .gitignore to prevent future staging
   3. Commit the changes
Error: Push blocked due to disallowed files
```

### ⚠️ HTTPS Warning
```bash
$ xg push origin main
🚀 xg: Checking staged files before push...
📤 Target: origin -> main
📋 Found 0 staged files
⚠️  You're using an HTTPS remote: https://github.com/user/repo.git
👉 It's recommended to use SSH for pushing to Git remotes.
   Example: git@github.com:user/repo.git
   See: https://docs.github.com/en/authentication/connecting-to-github-with-ssh
✅ All checks passed! Executing git push...
🚀 Successfully pushed to origin:main
```

---

## 🚫 Safety File Blocking

| File/Pattern | Description | Why Blocked |
|--------------|-------------|-------------|
| `.env` | Environment variables & secrets | Contains sensitive data like passwords |
| `node_modules/` | Node.js dependencies | Too large, should be installed via npm |
| `.idea/` | IntelliJ IDEA settings | IDE-specific, not needed in repo |
| `target/` | Rust build artifacts | Generated files, should be built locally |
| `.DS_Store` | macOS system files | OS-specific, not needed |
| `.vscode/` | VS Code settings | IDE-specific, not needed in repo |

> 💡 **Smart Blocking**: xg adapts blocking rules based on your project type for optimal safety.

---

## 🔧 Advanced Usage

### Using with Git Aliases
Add to your `.bashrc` or `.zshrc`:
```bash
alias gpush='git-guard push'
```

Then use:
```bash
gpush origin main
```

### Using with Git Wrapper (Already covered above)
The git wrapper setup is already covered in the [Basic Usage](#-basic-usage) section above. This is the recommended approach for seamless integration.

### Using in CI/CD
```bash
# In your CI pipeline
git-guard push origin main || {
  echo "Push blocked due to unwanted files"
  exit 1
}
```

---

## 🛠️ Troubleshooting

### Common Issues

#### "Not in a git repository"
```bash
Error: Not in a git repository. Please run this command from a git repository.
```
**Solution:** Navigate to a git repository directory.

#### "git-guard: command not found"
```bash
bash: git-guard: command not found
```
**Solution:** 
1. Make sure `$HOME/.cargo/bin` is in your PATH
2. Add to your shell config: `export PATH="$HOME/.cargo/bin:$PATH"`

#### "Failed to execute git push"
```bash
Error: Failed to execute git push: No such file or directory
```
**Solution:** Make sure git is installed and accessible in your PATH.

#### "Git push failed with exit code"
```bash
Error: Git push failed with exit code: 1
```
**Solution:** This is a normal git error (e.g., authentication issues, conflicts). Check your git configuration and remote setup.

### Getting Help
```bash
# Show all commands
git-guard --help

# Show push command help
git-guard push --help

# Check version
git-guard --version
```

---

## 🔒 Security Best Practices

### 1. Use SSH Instead of HTTPS
```bash
# Change from HTTPS to SSH
git remote set-url origin git@github.com:user/repo.git
```

### 2. Add Blocked Files to .gitignore
```bash
# Add to .gitignore
echo ".env" >> .gitignore
echo "node_modules/" >> .gitignore
echo ".idea/" >> .gitignore
echo "target/" >> .gitignore
echo ".DS_Store" >> .gitignore
echo ".vscode/" >> .gitignore
```

### 3. Remove Already Staged Blocked Files
```bash
# Remove from staging (but keep the file)
git rm --cached .env

# Commit the removal
git commit -m "Remove .env from tracking"
```

---

## 📝 Examples

### Development Workflow
```bash
# 1. Make changes
echo "new feature" >> src/feature.rs

# 2. Stage changes
git add src/feature.rs

# 3. Commit
git commit -m "Add new feature"

# 4. Push safely with git-guard
git-guard push origin main
```

### Testing Blocked Files
```bash
# Create a test file
echo "secret=password123" > .env

# Stage it
git add .env

# Try to push (will be blocked)
git-guard push origin main

# Remove the file
git rm --cached .env
rm .env

# Add to .gitignore
echo ".env" >> .gitignore

# Commit the changes
git add .gitignore
git commit -m "Add .env to gitignore"

# Now push should work
git-guard push origin main
```

### Force Push (Use with Caution)
```bash
# Force push after rebase
git-guard push origin main --force
```

---

## 🎯 Tips and Tricks

### 1. Always Use git-guard
Make it a habit to use `git-guard push` instead of `git push` to catch mistakes early.

### 2. Check Before Committing
```bash
# See what's staged
git status

# See what git-guard would check
git-guard push origin main
```

### 3. Use in Teams
Share this tool with your team to prevent accidental commits of sensitive files.

### 4. Customize for Your Project
Consider adding project-specific patterns to the blocked files list.

---

## 📚 Related Documentation

- [README.md](../README.md) - Project overview and quick start
- [LICENSE.md](../LICENSE.md) - MIT License

---

## 🤝 Contributing

Found a bug or want to add features? Check out the [Contributing Guide](../CONTRIBUTING.md) or open an issue on GitHub.