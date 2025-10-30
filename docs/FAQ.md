# ❓ xg Frequently Asked Questions

## 🤔 General Questions

### What is xg?
`xg` is a complete Git wrapper and project templating tool that enhances your Git workflow with safety features and instant project creation. It serves as both a safety net for Git operations and a powerful project scaffolding system.

### Why should I use xg?
- **🚀 Project Creation**: Instantly create projects with `xg init` and framework selection
- **🛡️ Git Safety**: Prevents embarrassing commits of sensitive files like `.env`
- **🔄 Complete Git Wrapper**: All git commands work with enhanced safety
- **⚡ Zero Friction**: Drop-in replacement for git with added intelligence
- **🎯 Interactive Prompts**: Choose frameworks, dependencies, and configurations

### How is xg different from regular git?
- **Safety First**: Blocks unwanted files before push
- **Project Templating**: Creates complete project structures instantly
- **Enhanced Feedback**: Better error messages and suggestions
- **Framework Intelligence**: Adapts behavior based on project type
- **Zero Breaking Changes**: All existing git workflows continue to work

### Is xg a replacement for git?
No, `xg` enhances git without replacing it. It acts as a wrapper that adds safety features and project templating while maintaining full git compatibility.

---

## 🚀 Installation & Setup

### How do I install xg?
```bash
git clone https://github.com/nishujangra/git-guard.git
cd git-guard
cargo install --path .
```

### I get "xg: command not found" after installation
Add `$HOME/.cargo/bin` to your PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
Add this line to your `~/.bashrc` or `~/.zshrc` for persistence.

### Can I install xg without Rust?
No, `xg` is written in Rust and requires Rust to compile. You can install Rust from [rustup.rs](https://rustup.rs/).

### Does xg work on Windows?
Yes! It works on Windows, macOS, and Linux. On Windows, we recommend using WSL (Windows Subsystem for Linux) for the best experience.

### How do I use xg as my default git command?
```bash
# Add to your ~/.bashrc or ~/.zshrc
echo 'alias git="xg"' >> ~/.bashrc
source ~/.bashrc

# Now 'git' commands automatically use xg
git init -lang rust -name "myproject"  # Works!
```

---

## 🔧 Usage

### How do I create a new project with xg?
```bash
# Interactive project creation
xg init -lang golang -name "my-api"
# Prompts: Which framework? (echo/gin/fiber)

# Direct framework selection
xg init -lang javascript -name "my-app" -f react
```

### How do I use xg for Git operations?
```bash
# Option 1: Use xg directly
xg status
xg add .
xg commit -m "feat: add new feature"
xg push origin main

# Option 2: Replace git globally
alias git="xg"
git push origin main  # Now uses xg automatically
```

### Can I use xg with force push?
Yes! `xg` passes through all git arguments:
```bash
xg push origin main --force
xg push origin main --tags
```

### Does xg work with all git remotes?
Yes, `xg` works with any git remote (GitHub, GitLab, Bitbucket, etc.).

### Can I use xg in CI/CD pipelines?
Yes! `xg` can be used in CI/CD to prevent unwanted files and ensure project consistency:
```bash
xg push origin main || {
  echo "Push blocked due to unwanted files"
  exit 1
}
```

---

## 🎯 Project Templating

### What languages and frameworks does xg support?
xg supports multiple languages with various frameworks:

**Go**: Echo, Gin, Fiber, Chi, Standard Library
**JavaScript/TypeScript**: React, Vue, Svelte, Node.js, Express, Fastify
**Rust**: CLI tools, libraries, web apps, API servers
**Python**: FastAPI, Flask, Django, data science stacks

### Can I customize the generated projects?
Yes! xg templates include:
- **Configuration files** (.env, docker-compose.yml)
- **Build scripts** (Makefile, package.json scripts)
- **Documentation** (README.md)
- **Testing setup** (basic test files)

### How do I add a new framework to xg?
You can contribute new templates by:
1. Creating template files in the `templates/` directory
2. Adding configuration metadata
3. Submitting a pull request

### Does xg generate Docker files?
Yes! Most templates include:
- **Dockerfile** for containerization
- **docker-compose.yml** for local development
- **Multi-stage builds** for optimization

---

## 🚫 File Blocking

### What files does xg block?
Currently blocks:
- `.env` - Environment variables & secrets
- `node_modules/` - Node.js dependencies
- `.idea/` - IntelliJ IDEA settings
- `target/` - Rust build artifacts
- `.DS_Store` - macOS system files
- `.vscode/` - VS Code settings

### Can I customize which files are blocked?
Not yet, but this feature is planned for future versions. For now, the patterns are hardcoded to cover the most common unwanted files.

### Why does xg block `.env` files?
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

### Does xg check unstaged files?
No, `xg` only checks staged files (files that have been `git add`ed). This is intentional because only staged files will be pushed.

---

## ⚠️ HTTPS Warnings

### Why does xg warn about HTTPS remotes?
HTTPS remotes require you to enter your password/token on every push, which is:
- **Less secure** than SSH keys
- **More inconvenient** for frequent pushes
- **Prone to credential exposure**

### How do I switch from HTTPS to SSH?
```bash
# Check current remote
xg remote -v

# Change to SSH
xg remote set-url origin git@github.com:user/repo.git

# Verify the change
xg remote -v
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

### How do I make xg my default git command?
The best way is to alias `git` to `xg` globally:

```bash
# Add to your ~/.bashrc or ~/.zshrc
echo 'alias git="xg"' >> ~/.bashrc
source ~/.bashrc
```

**Now you can use git normally with enhanced features:**
```bash
git init -lang golang -name "my-api"  # Project creation!
git push origin main                  # Safe push with blocking
git status                           # Enhanced status
git commit -m "feat: add feature"    # Normal commits
```

**Why use the alias approach?**
- **Zero friction** - All git commands work identically
- **Enhanced features** - Access to `xg init` and safety features
- **Team adoption** - Easy to recommend to team members
- **Habit formation** - Becomes part of your normal workflow

### Can I use xg with git aliases?
Yes! Add to your git config:
```bash
git config --global alias.safepush '!xg push'
```

Then use:
```bash
git safepush origin main
```

### Does xg work with git hooks?
Yes, you can use `xg` in pre-push hooks, but it's usually not necessary since `xg` already provides the same protection when used as a git wrapper.

---

## 🚀 Project Creation Workflows

### What's the typical workflow for starting a new project?
```bash
# 1. Create the project
xg init -lang golang -name "user-service"

# 2. Navigate to the project
cd user-service

# 3. Install dependencies
go mod tidy

# 4. Run the project
go run main.go

# 5. Make it a git repository (if not already)
xg add .
xg commit -m "Initial commit"
```

### Can I create projects in existing directories?
No, `xg init` creates new directories. For existing projects, use regular git commands with xg as a wrapper.

### How do I customize generated projects?
Currently, xg generates standardized templates. Future versions will support:
- Custom templates
- Configuration overrides
- Template variables

### What if I don't like the generated structure?
You can modify the generated files as needed. The templates provide a solid starting point that you can customize for your needs.

---

## 🔮 Future Features

### Will xg support custom templates?
Yes, this is planned for future versions. You'll be able to create and share custom project templates.

### Will xg support more languages?
Yes! Planned languages include:
- Java (Spring Boot, Quarkus)
- C# (.NET)
- PHP (Laravel, Symfony)
- Ruby (Rails)
- And more based on community demand

### Will xg have a GUI?
No plans for a GUI. `xg` is designed to be a powerful CLI tool that integrates seamlessly with existing workflows.

---

## 🔒 Security

### Is xg secure?
Yes, `xg` is:
- **Open source** - code can be audited
- **Written in Rust** - memory safe and secure
- **Minimal dependencies** - reduces attack surface
- **Local only** - doesn't send data anywhere

### Does xg send my data anywhere?
No, `xg` runs entirely locally and doesn't send any data to external services.

### Can xg access my git credentials?
No, `xg` only reads git status and executes git commands. It doesn't access or store any credentials.

---

## 🚀 Performance

### How fast is xg?
`xg` is very fast:
- **Compilation**: ~5-10 seconds (one-time)
- **Execution**: ~100-200ms per command
- **Project creation**: ~2-5 seconds for typical projects
- **Memory usage**: ~5-10MB

### Does xg slow down my workflow?
Minimally. The overhead is typically less than 200ms, which is negligible compared to the time saved by preventing unwanted commits and speeding up project setup.

---

## 🤝 Contributing

### How can I contribute to xg?
1. **Report bugs** - Open an issue on GitHub
2. **Suggest features** - Create a feature request (templating, new languages, etc.)
3. **Submit code** - Fork the repo and create a pull request
4. **Improve docs** - Help make the documentation better
5. **Add templates** - Create new language/framework templates

### What should I include in a bug report?
- Operating system and version
- Rust version (`rustc --version`)
- Git version (`git --version`)
- Steps to reproduce the issue
- Expected vs actual behavior
- Error messages (if any)

### Can I add new templates or languages?
Yes! You can contribute by:
- Adding new language templates in the `templates/` directory
- Creating framework variations for existing languages
- Improving existing templates
- Adding new blocked file patterns in `src/rules.rs`

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