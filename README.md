# 🚀 xg - Safe Git with superpowers

**xg (ex-gee)** is a complete Git wrapper and project templating tool that combines the power of Git with intelligent project scaffolding and safety features.

**Like neovim is to vim, xg is to git - upgraded, modern, and feature-rich!**

`or use as alias git = xg`

---

## 🚀 Quick About the Project

`xg` is the next evolution of Git tooling. It serves as:

1. **Complete Git Wrapper** - All git commands work through xg with enhanced safety
2. **Project Templating Engine** - Create new projects with `xg init` and interactive framework selection
3. **Safety Net** - Prevents accidental pushes of sensitive files and build artifacts

### ⭐ Key Features

- **🚀 Project Creation**: `xg init -lang golang -name "myproject"` with framework selection
- **🛡️ Git Safety**: Blocks unwanted files (`.env`, `node_modules/`, build artifacts)
- **🔄 Full Git Wrapper**: All git commands work through xg (`xg status`, `xg commit`, etc.)
- **🎯 Interactive Prompts**: Choose frameworks, dependencies, and configurations
- **⚡ Zero Friction**: Drop-in replacement for git commands

### ⭐ Recommended: Replace Git with xg

Make xg your default git command for enhanced safety and productivity:

```bash
# Add to your shell config (~/.bashrc, ~/.zshrc, etc.)
echo 'alias git="xg"' >> ~/.bashrc
source ~/.bashrc
```

**Now use Git normally with enhanced features:**
```bash
git push origin main     # Automatically protected by xg!
git status              # Enhanced status with safety indicators
git commit -m "msg"     # Works normally
git log                 # Works normally
git init -lang rust -name "myproject"  # Create new project with templates!
```

**Benefits of using xg as git:**
- ✅ **Zero friction** - All git commands work exactly the same
- ✅ **Automatic protection** - Every push is safety-checked
- ✅ **Project templating** - Access to `git init` with frameworks
- ✅ **Enhanced feedback** - Better error messages and suggestions

### 🔄 Usage Modes

**Project Creation:**
```bash
xg init -lang golang -name "api-server"
# Interactive: Which framework? (echo/gin/fiber)
# Creates complete project structure with chosen framework
```

**Git Operations (all work normally):**
```bash
xg status              # Shows status
xg add .               # Stages files
xg commit -m "feat: add new feature"  # Commits
xg push origin main    # Safe push with file blocking
xg pull               # Normal pull
xg branch             # Branch management
```

**Direct Usage:**
- **Safe pushing:** `xg push origin main`
- **CI/CD integration:** Use in deployment pipelines
- **Team adoption:** Share with your team as the new git standard

---

## 📚 Documentation

For full usage, installation, and troubleshooting, see:
- [📖 Complete Usage Guide](docs/USAGE.md)
- [🚀 Project Initialization Guide](docs/INIT.md)
- [🔧 Installation Guide](docs/INSTALLATION.md)
- [🎯 Supported Templates](docs/TEMPLATES.md)
- [❓ FAQ](docs/FAQ.md)

---

## 🚫 Safety Features

`xg` includes intelligent file blocking to prevent accidental commits of sensitive or unwanted files:

| File/Pattern    | What it is                    | Why it's blocked                        |
|-----------------|-------------------------------|-----------------------------------------|
| `.env`          | Environment variables & secrets| Contains sensitive data like passwords  |
| `node_modules/` | Node.js dependencies          | Too large, should be installed via npm  |
| `.idea/`        | IntelliJ IDEA settings        | IDE-specific, not needed in repo        |
| `target/`       | Rust build artifacts          | Generated files, should be built locally|
| `.DS_Store`     | macOS system files            | OS-specific, not needed                 |
| `.vscode/`      | VS Code settings              | IDE-specific, not needed in repo        |

> 💡 **Smart Protection**: xg analyzes your project type and applies appropriate blocking rules. If you accidentally stage these files, xg will catch them and guide you on how to fix it.

## 🎯 Project Templates

xg supports instant project creation for multiple languages and frameworks:

### Quick Examples
```bash
# Go API Server
xg init -lang golang -name "api-server"
# Prompts: Framework? (echo/gin/fiber) -> Creates complete project

# React Application
xg init -lang javascript -name "react-app"
# Prompts: Framework? (react/vue/svelte) -> Sets up with Vite/Webpack

# Rust CLI Tool
xg init -lang rust -name "cli-tool"
# Creates Cargo.toml, src/main.rs, and basic structure
```

### Supported Languages
- **Go**: echo, gin, fiber frameworks
- **JavaScript/TypeScript**: React, Vue, Svelte, Node.js
- **Rust**: CLI tools, libraries, web apps
- **Python**: FastAPI, Flask, Django
- **And more coming soon!**

## 📜 License

MIT

---

## 💡 Author & Community

Made with ❤️ by [Nishant](https://github.com/nishujangra)

### 🤝 Contributing
We welcome contributions! Whether it's:
- Adding new language templates
- Improving existing templates
- Enhancing safety features
- Documentation improvements

Check out our [Contributing Guide](CONTRIBUTING.md) to get started.

### 🌟 Star the Project
If xg helps your workflow, please give it a star on GitHub! ⭐