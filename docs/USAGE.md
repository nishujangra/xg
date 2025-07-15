# 📖 git-guard Complete Usage Guide

## 🚀 Quick Installation

### Prerequisites
- **Rust** (install from [rustup.rs](https://rustup.rs/))
- **Git** (should already be installed)

### Install git-guard
```bash
# Clone the repository
git clone https://github.com/nishujangra/git-guard.git
cd git-guard

# Install manually
cargo install --path .
```

### Make git-guard Available Globally

If you get "command not found" after installation, add Cargo's bin directory to your PATH:

```bash
# Add to your shell config (~/.bashrc, ~/.zshrc, etc.)
export PATH="$HOME/.cargo/bin:$PATH"

# Reload your shell
source ~/.bashrc  # or ~/.zshrc

# Now you can use git-guard from anywhere
git-guard --version
```

### Verify Installation
```bash
git-gaurd --version
# Should show: git-guard 0.1.0
```

---

## 📋 Basic Usage

### ⭐ Recommended: Use as Git Wrapper (Best Practice)

**Set up git-guard as a wrapper so `git push` automatically uses git-guard:**

```bash
# Create ~/bin/git-wrapper
touch ~/bin/git-wrapper

cat > ~/bin/git-wrapper << 'EOF'
#!/bin/bash
if [[ "$1" == "push" ]]; then
  shift
  git-guard push "$@"
else
  command git "$@"
fi
EOF

# Make it executable
chmod +x ~/bin/git-wrapper

# Add to your shell config (~/.bashrc, ~/.zshrc, etc.)
echo 'alias git="git-wrapper"' >> ~/.bashrc

# Reload your shell
source ~/.bashrc
```

**Now you can use git normally:**
```bash
git push origin main  # Automatically uses git-guard!
git status           # Works normally
git commit -m "msg"  # Works normally
```

### Alternative: Direct Usage

If you prefer to use git-guard directly:

Instead of:
```bash
git push origin main
```

Use:
```bash
git-guard push origin main
```

### Command Syntax
```bash
git-guard push <remote> <branch> [additional-git-args...]
```

**Examples:**
```bash
# Basic push
git-guard push origin main

# Force push
git-guard push origin main --force

# Push with tags
git-guard push origin main --tags

# Push specific branch
git-guard push origin feature/new-feature
```

---

## 🎯 What git-guard Does

### 1. **Repository Check**
- Verifies you're in a git repository
- Exits with error if not in a git repo

### 2. **Staged File Analysis**
- Shows all files you're about to push
- Lists them clearly with emoji indicators

### 3. **File Blocking**
- Checks for unwanted files (`.env`, `node_modules/`, etc.)
- Blocks push if any blocked files are found
- Shows exactly which files are blocked and why

### 4. **Security Warnings**
- Warns about HTTPS remotes (recommends SSH)
- Provides helpful links for SSH setup

### 5. **Git Push Execution**
- Executes the actual `git push` command
- Shows git's output and success/failure status

---

## 📊 Example Outputs

### ✅ Successful Push (Clean Repository)
```bash
$ git-guard push origin main
🚀 git-guard: Checking staged files before push...
📤 Target: origin -> main
📋 Found 0 staged files
✅ All checks passed! Executing git push...
🚀 Successfully pushed to origin:main
```

### ✅ Successful Push (With Staged Files)
```bash
$ git-guard push origin main
🚀 git-guard: Checking staged files before push...
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
$ git-guard push origin main
🚀 git-guard: Checking staged files before push...
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
   1. Remove blocked files: git rm --cached <file>
   2. Add to .gitignore to prevent future staging
   3. Commit the changes
Error: Push blocked due to disallowed files
```

### ⚠️ HTTPS Warning
```bash
$ git-guard push origin main
🚀 git-guard: Checking staged files before push...
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

## 🚫 Blocked File Types

| File/Pattern | Description | Why Blocked |
|--------------|-------------|-------------|
| `.env` | Environment variables & secrets | Contains sensitive data like passwords |
| `node_modules/` | Node.js dependencies | Too large, should be installed via npm |
| `.idea/` | IntelliJ IDEA settings | IDE-specific, not needed in repo |
| `target/` | Rust build artifacts | Generated files, should be built locally |
| `.DS_Store` | macOS system files | OS-specific, not needed |
| `.vscode/` | VS Code settings | IDE-specific, not needed in repo |

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