# xg Frequently Asked Questions

## General Questions

### What is xg?
xg is a developer tool that provides Git safety features and project templating capabilities. It acts as a wrapper around Git commands with built-in safeguards, and includes generators for quickly scaffolding new projects.

### Why should I use xg?
- Project Creation: Generate complete project structures with framework-specific templates
- Git Safety: Prevent accidental commits of sensitive files like `.env`
- Git Wrapper: All git commands work with enhanced safety checks
- Zero Friction: Drop-in replacement for git with added intelligence
- Interactive Prompts: Choose frameworks, dependencies, and configurations

### How is xg different from regular git?
- Safety First: Blocks unwanted files before push operations
- Project Templating: Creates complete project structures instantly
- Enhanced Feedback: Better error messages and suggestions
- Framework Intelligence: Adapts behavior based on project type
- Zero Breaking Changes: Existing git workflows continue to work

### Is xg a replacement for git?
No, xg enhances git without replacing it. It acts as a wrapper that adds safety features and project templating while maintaining full git compatibility.

## Installation & Setup

### How do I install xg?
```bash
git clone https://github.com/nishujangra/xg
cd xg
cargo install --path .
```

### I get "xg: command not found" after installation
Add `$HOME/.cargo/bin` to your PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
Add this line to your `~/.bashrc` or `~/.zshrc` for persistence.

### Can I install xg without Rust?
No, xg is written in Rust and requires Rust to compile. You can install Rust from [rustup.rs](https://rustup.rs/).

### Does xg work on Windows?
Yes. It works on Windows, macOS, and Linux. On Windows, WSL (Windows Subsystem for Linux) provides the best experience.

### How do I use xg as my default git command?
```bash
# Add to your ~/.bashrc or ~/.zshrc
echo 'alias git="xg"' >> ~/.bashrc
source ~/.bashrc

# Now 'git' commands automatically use xg
git status  # Enhanced status with safety checks
```

## Usage

### How do I create a new project with xg?
```bash
# Generate a Go project
xg create-go-app --project "my-api" --rest-framework gin

# This creates a complete project structure with:
# - Idiomatic Go folder layout
# - Configuration files
# - Framework bootstrap code
# - Go module initialization
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
Yes. xg passes through all git arguments:
```bash
xg push origin main --force
xg push origin main --tags
```

### Does xg work with all git remotes?
Yes, xg works with any git remote (GitHub, GitLab, Bitbucket, etc.).

### Can I use xg in CI/CD pipelines?
Yes. xg can be used in CI/CD to prevent unwanted files and ensure project consistency:
```bash
xg push origin main || {
  echo "Push blocked due to unwanted files"
  exit 1
}
```

## Project Templating

### What languages and frameworks does xg support?
xg currently supports Go project generation with:

Go: Echo, Gin

Additional language support is planned for future releases.

### Can I customize the generated projects?
Yes. xg templates include:
- Configuration files (.env, docker-compose.yml)
- Build scripts (Makefile, package.json scripts)
- Documentation (README.md)
- Testing setup (basic test files)

### How do I add a new framework to xg?
You can contribute new templates by:
1. Creating template files in the `src/generators/` directory
2. Adding generator logic
3. Submitting a pull request

### Does xg generate Docker files?
Yes. Most templates include:
- Dockerfile for containerization
- docker-compose.yml for local development
- Multi-stage builds for optimization

## File Blocking

### What files does xg block?
Currently blocks:
- `.env` - Environment variables and secrets
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
1. Rename it to something that doesn't match the pattern
2. Use a different approach (like environment variables)
3. Wait for configuration support in future versions

### Does xg check unstaged files?
No, xg only checks staged files (files that have been `git add`ed). This is intentional because only staged files will be pushed.

## HTTPS Warnings

### Why does xg warn about HTTPS remotes?
HTTPS remotes require you to enter your password/token on every push, which is:
- Less secure than SSH keys
- More inconvenient for frequent pushes
- Prone to credential exposure

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

## Troubleshooting

### "Not in a git repository" error
Problem: You're not in a git repository directory.

Solution: Navigate to a git repository:
```bash
cd your-git-repository
xg push origin main
```

### "Failed to execute git push" error
Problem: Git is not installed or not accessible.

Solution: Install or fix git installation:
```bash
# Check if git is installed
git --version

# Install git if needed
sudo apt-get install git  # Ubuntu/Debian
brew install git          # macOS
```

### "Git push failed with exit code" error
Problem: This is a normal git error (authentication, conflicts, etc.).

Solution: Check your git configuration:
```bash
# Check git status
git status

# Check remote configuration
git remote -v

# Check authentication
git push origin main  # Test without xg
```

### xg is slow
Problem: First run might be slow due to compilation.

Solution: This is normal for the first run. Subsequent runs will be fast.

## Workflow Integration

### How do I make xg my default git command?
The best way is to alias `git` to `xg` globally:

```bash
# Add to your ~/.bashrc or ~/.zshrc
echo 'alias git="xg"' >> ~/.bashrc
source ~/.bashrc
```

Now you can use git normally with enhanced features:
```bash
git status                           # Enhanced status
git push origin main                 # Safe push with blocking
git commit -m "feat: add feature"    # Normal commits
```

Why use the alias approach?
- Zero friction - All git commands work identically
- Enhanced features - Access to safety features
- Team adoption - Easy to recommend to team members
- Habit formation - Becomes part of your normal workflow

### Can I use xg with git aliases?
Yes. Add to your git config:
```bash
git config --global alias.safepush '!xg push'
```

Then use:
```bash
git safepush origin main
```

### Does xg work with git hooks?
Yes, you can use `xg` in pre-push hooks, but it's usually not necessary since `xg` already provides the same protection when used as a git wrapper.

## Project Creation Workflows

### What's the typical workflow for starting a new project?
```bash
# 1. Create the project
xg create-go-app --project "user-service" --rest-framework gin

# 2. Navigate to the project
cd user-service

# 3. Install dependencies
go mod tidy

# 4. Run the project
go run cmd/main.go
```

### Can I create projects in existing directories?
No, project generation creates new directories. For existing projects, use regular git commands with xg as a wrapper.

### How do I customize generated projects?
Currently, xg generates standardized templates. Future versions will support:
- Custom templates
- Configuration overrides
- Template variables

### What if I don't like the generated structure?
You can modify the generated files as needed. The templates provide a solid starting point that you can customize for your needs.

## Future Features

### Will xg support custom templates?
Yes, this is planned for future versions. You'll be able to create and share custom project templates.

### Will xg support more languages?
Yes. Planned languages include:
- Java (Spring Boot, Quarkus)
- C# (.NET)
- PHP (Laravel, Symfony)
- Ruby (Rails)
- And more based on community demand

### Will xg have a GUI?
No plans for a GUI. xg is designed to be a powerful CLI tool that integrates seamlessly with existing workflows.

## Security

### Is xg secure?
Yes, xg is:
- Open source - code can be audited
- Written in Rust - memory safe and secure
- Minimal dependencies - reduces attack surface
- Local only - doesn't send data anywhere

### Does xg send my data anywhere?
No, xg runs entirely locally and doesn't send any data to external services.

### Can xg access my git credentials?
No, xg only reads git status and executes git commands. It doesn't access or store any credentials.

## Performance

### How fast is xg?
xg is very fast:
- Compilation: ~5-10 seconds (one-time)
- Execution: ~100-200ms per command
- Project creation: ~2-5 seconds for typical projects
- Memory usage: ~5-10MB

### Does xg slow down my workflow?
Minimally. The overhead is typically less than 200ms, which is negligible compared to the time saved by preventing unwanted commits and speeding up project setup.

## Contributing

### How can I contribute to xg?
1. Report bugs - Open an issue on GitHub
2. Suggest features - Create a feature request (templating, new languages, etc.)
3. Submit code - Fork the repo and create a pull request
4. Improve docs - Help make the documentation better
5. Add templates - Create new language/framework templates

### What should I include in a bug report?
- Operating system and version
- Rust version (`rustc --version`)
- Git version (`git --version`)
- Steps to reproduce the issue
- Expected vs actual behavior
- Error messages (if any)

### Can I add new templates or languages?
Yes. You can contribute by:
- Adding new language templates in the `src/generators/` directory
- Creating framework variations for existing languages
- Improving existing templates