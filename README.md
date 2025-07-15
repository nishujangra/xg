# 🛡️ git-guard

A lightweight Rust CLI tool that prevents you from accidentally pushing unwanted files to your Git repository.

**Stop embarrassing commits before they leave your machine!** 

---

## 🚀 Quick About the Project

`git-guard` is a **safety net** for your Git workflow. It catches common mistakes before they become embarrassing commits.

- **Prevents accidental pushes** of sensitive files (`.env`, secrets)
- **Keeps repositories clean** by blocking build artifacts and IDE files
- **Educates developers** about security best practices
- **Integrates seamlessly** with existing Git workflows

### ⭐ Recommended: Use as a Git Wrapper

Make `git-guard` invisible to your workflow by wrapping your `git` command. This way, every `git push` automatically uses git-guard for protection:

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

# Add to your shell config
echo 'alias git="git-wrapper"' >> ~/.bashrc
source ~/.bashrc
```

**Now use Git normally:**
```bash
git push origin main  # Automatically protected by git-guard!
git status           # Works normally
git commit -m "msg"  # Works normally
git log              # Works normally
```

**Benefits of the wrapper approach:**
- ✅ **Zero friction** - No need to remember different commands
- ✅ **Automatic protection** - Every push is checked
- ✅ **Team adoption** - Easy to recommend to team members
- ✅ **Habit formation** - Becomes part of normal git workflow

### 🔄 Other Uses
- **Direct usage:** `git-guard push origin main`
- **CI/CD integration:** Use in deployment pipelines to block unwanted files
- **Team adoption:** Share with your team, or set up as a git alias
- **Pre-push hook:** Add to `.git/hooks/pre-push` for extra safety

---

## 📚 Documentation

For full usage, installation, and troubleshooting, see:
- [📖 Usage Guide](docs/USAGE.md)
- [🔧 Installation Guide](docs/INSTALLATION.md)
- [❓ FAQ](docs/FAQ.md)

---

## 🚫 What Files Are Blocked?

`git-guard` prevents you from pushing these common unwanted files:

| File/Pattern    | What it is                    | Why it's blocked                        |
|-----------------|-------------------------------|-----------------------------------------|
| `.env`          | Environment variables & secrets| Contains sensitive data like passwords  |
| `node_modules/` | Node.js dependencies          | Too large, should be installed via npm  |
| `.idea/`        | IntelliJ IDEA settings        | IDE-specific, not needed in repo        |
| `target/`       | Rust build artifacts          | Generated files, should be built locally|
| `.DS_Store`     | macOS system files            | OS-specific, not needed                 |
| `.vscode/`      | VS Code settings              | IDE-specific, not needed in repo        |

> 💡 **Don't worry!** If you accidentally stage these files, `git-guard` will catch them and show you exactly how to fix it.

## 📜 License

MIT

---

## 💡 Author

Made by [Nishant](https://github.com/nishujangra)