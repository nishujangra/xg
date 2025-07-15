# ❓ git-guard Frequently Asked Questions

## 🤔 General Questions

### What is git-guard?
`git-guard` is a Rust CLI tool that prevents you from accidentally pushing unwanted files (like `.env`, `node_modules/`, etc.) to your Git repository. It acts as a safety net between your local commits and the remote repository.

### Why should I use git-guard?
- **Prevents embarrassing commits** of sensitive files like `.env`
- **Keeps repositories clean** by blocking build artifacts and IDE files
- **Educational** - teaches you about security best practices
- **Fast and lightweight** - minimal overhead on your workflow

### How is it different from .gitignore?
- **`.gitignore`** prevents files from being staged in the first place
- **`git-guard`** catches files that were already staged and blocks the push
- **`git-guard`** is a last line of defense when `.gitignore` fails or is forgotten

### Is git-guard a replacement for git push?
No, `git-guard` is a wrapper around `git push` that adds safety checks. It still executes the actual `git push` command when all checks pass.

---

## 🚀 Installation & Setup

### How do I install git-guard?
```bash
git clone https://github.com/nishujangra/git-guard.git
cd git-guard
cargo install --path .
```

### I get "git-guard: command not found" after installation
Add `$HOME/.cargo/bin` to your PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
Add this line to your `~/.bashrc` or `~/.zshrc` for persistence.

### Can I install git-guard without Rust?
No, `git-guard` is written in Rust and requires Rust to compile. You can install Rust from [rustup.rs](https://rustup.rs/).

### Does git-guard work on Windows?
Yes! It works on Windows, macOS, and Linux. On Windows, we recommend using WSL (Windows Subsystem for Linux) for the best experience.

---

## 🔧 Usage

### How do I use git-guard?
Instead of `git push origin main`, use:
```bash
git-guard push origin main
```

### Can I use git-guard with force push?
Yes! `git-guard` passes through all git arguments:
```bash
git-guard push origin main --force
```

### Does git-guard work with all git remotes?
Yes, `git-guard` works with any git remote (GitHub, GitLab, Bitbucket, etc.).

### Can I use git-guard in CI/CD pipelines?
Yes! `git-guard` can be used in CI/CD to prevent unwanted files from being pushed:
```bash
git-guard push origin main || {
  echo "Push blocked due to unwanted files"
  exit 1
}
```

---

## 🚫 File Blocking

### What files does git-guard block?
Currently blocks:
- `.env` - Environment variables & secrets
- `node_modules/` - Node.js dependencies
- `.idea/` - IntelliJ IDEA settings
- `target/` - Rust build artifacts
- `.DS_Store` - macOS system files
- `.vscode/` - VS Code settings

### Can I customize which files are blocked?
Not yet, but this feature is planned for future versions. For now, the patterns are hardcoded to cover the most common unwanted files.

### Why does git-guard block `.env` files?
`.env` files typically contain sensitive information like:
- Database passwords
- API keys
- Access tokens
- Configuration secrets

These should never be committed to version control.

### What if I need to commit a file that's blocked?
If you have a legitimate reason to commit a blocked file:
1. **Rename it** to something that doesn't match the pattern
2. **Use a different approach** (like environment variables)
3. **Wait for configuration support** in future versions

### Does git-guard check unstaged files?
No, `git-guard` only checks staged files (files that have been `git add`ed). This is intentional because only staged files will be pushed.

---

## ⚠️ HTTPS Warnings

### Why does git-guard warn about HTTPS remotes?
HTTPS remotes require you to enter your password/token on every push, which is:
- **Less secure** than SSH keys
- **More inconvenient** for frequent pushes
- **Prone to credential exposure**

### How do I switch from HTTPS to SSH?
```bash
# Check current remote
git remote -v

# Change to SSH
git remote set-url origin git@github.com:user/repo.git

# Verify the change
git remote -v
```

### Can I ignore the HTTPS warning?
Yes, the HTTPS warning is just a recommendation. Your push will still work with HTTPS remotes.

---

## 🛠️ Troubleshooting

### "Not in a git repository" error
**Problem**: You're not in a git repository directory.

**Solution**: Navigate to a git repository:
```bash
cd your-git-repository
git-guard push origin main
```

### "Failed to execute git push" error
**Problem**: Git is not installed or not accessible.

**Solution**: Install or fix git installation:
```bash
# Check if git is installed
git --version

# Install git if needed
sudo apt-get install git  # Ubuntu/Debian
brew install git          # macOS
```

### "Git push failed with exit code" error
**Problem**: This is a normal git error (authentication, conflicts, etc.).

**Solution**: Check your git configuration:
```bash
# Check git status
git status

# Check remote configuration
git remote -v

# Check authentication
git push origin main  # Test without git-guard
```

### git-guard is slow
**Problem**: First run might be slow due to compilation.

**Solution**: This is normal for the first run. Subsequent runs will be fast.

---

## 🔄 Workflow Integration

### How do I make git-guard the default for git push?
The best way is to use git-guard as a wrapper around your `git` command. This makes it invisible to your workflow:

```bash
# Create the wrapper script
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
source ~/.bashrc
```

**Now you can use git normally:**
```bash
git push origin main  # Automatically uses git-guard!
git status           # Works normally
git commit -m "msg"  # Works normally
```

**Why use the wrapper approach?**
- **Zero friction** - No need to remember to use `git-guard push`
- **Automatic protection** - Every push is checked without thinking about it
- **Team adoption** - Easy to recommend to team members
- **Habit formation** - Becomes part of your normal git workflow

### Can I use git-guard with git aliases?
Yes! Add to your git config:
```bash
git config --global alias.safepush '!git-guard push'
```

Then use:
```bash
git safepush origin main
```

### Does git-guard work with git hooks?
Yes, you can use `git-guard` in pre-push hooks, but it's usually not necessary since `git-guard` already provides the same protection.

---

## 🔒 Security

### Is git-guard secure?
Yes, `git-guard` is:
- **Open source** - code can be audited
- **Written in Rust** - memory safe and secure
- **Minimal dependencies** - reduces attack surface
- **Local only** - doesn't send data anywhere

### Does git-guard send my data anywhere?
No, `git-guard` runs entirely locally and doesn't send any data to external services.

### Can git-guard access my git credentials?
No, `git-guard` only reads git status and executes git commands. It doesn't access or store any credentials.

---

## 🚀 Performance

### How fast is git-guard?
`git-guard` is very fast:
- **Compilation**: ~5-10 seconds (one-time)
- **Execution**: ~100-200ms per push
- **Memory usage**: ~5-10MB

### Does git-guard slow down my workflow?
Minimally. The overhead is typically less than 200ms, which is negligible compared to the time saved by preventing unwanted commits.

---

## 🔮 Future Features

### Will git-guard support custom patterns?
Yes, this is planned for future versions. You'll be able to configure your own blocked file patterns.

### Will git-guard support different rule sets per project?
Yes, this is planned. You'll be able to have project-specific blocking rules.

### Will git-guard support other git commands?
Possibly. We're considering support for `git commit` and other commands that could benefit from file checking.

### Will git-guard have a GUI?
No plans for a GUI. `git-guard` is designed to be a lightweight CLI tool that integrates seamlessly with existing workflows.

---

## 🤝 Contributing

### How can I contribute to git-guard?
1. **Report bugs** - Open an issue on GitHub
2. **Suggest features** - Create a feature request
3. **Submit code** - Fork the repo and create a pull request
4. **Improve docs** - Help make the documentation better

### What should I include in a bug report?
- Operating system and version
- Rust version (`rustc --version`)
- Git version (`git --version`)
- Steps to reproduce the issue
- Expected vs actual behavior
- Error messages (if any)

### Can I add new blocked file patterns?
Yes! You can submit a pull request to add new patterns to the `DEFAULT_BLOCKED_PATTERNS` in `src/rules.rs`.

---

## 📚 More Help

### Where can I get more help?
- **Documentation**: Check the [Usage Guide](USAGE.md) and [Installation Guide](INSTALLATION.md)
- **GitHub Issues**: Search existing issues or create a new one
- **GitHub Discussions**: Ask questions in the Discussions tab

### How do I report a bug?
1. Search existing issues to see if it's already reported
2. Create a new issue with:
   - Clear title describing the problem
   - Detailed description of the issue
   - Steps to reproduce
   - System information
   - Error messages

### How do I request a feature?
1. Search existing issues to see if it's already requested
2. Create a new issue with:
   - Clear title describing the feature
   - Detailed description of the use case
   - Examples of how it would work
   - Benefits of the feature