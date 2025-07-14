# 🛡️ git-guard

A lightweight Rust CLI tool that prevents you from accidentally pushing unwanted files to your Git repository.

**Stop embarrassing commits before they leave your machine!** 🚫

## 🎯 What does it do?

Ever accidentally committed `.env` files with secrets, or pushed `node_modules/` to your repo? `git-guard` catches these mistakes before they reach the remote repository.

Instead of `git push`, use `git-guard push` and it will:
- ✅ Check what files you're about to push
- 🚫 Block the push if it finds unwanted files
- ⚠️ Warn you about security issues (like HTTPS remotes)
- 📝 Show you exactly what's wrong and how to fix it

---

## 🚀 Features

- ✅ **Smart File Detection** - Automatically finds unwanted files in your staged changes
- ✅ **Clear Blocking** - Stops the push and shows exactly what's wrong
- ✅ **Helpful Guidance** - Tells you how to fix the problem step by step
- ✅ **Security Warnings** - Alerts you about HTTPS remotes (SSH is safer)
- ✅ **Fast & Reliable** - Written in Rust for speed and reliability
- 🔄 **Git Integration** - Works seamlessly with your existing git workflow

---

## 🚀 Quick Start

### Step 1: Install Rust (if you don't have it)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2: Install git-guard
```bash
git clone https://github.com/nishujangra/git-guard.git
cd git-guard
cargo install --path .
```

That's it! `git-guard` is now installed and ready to use.

---

## 📖 How to Use

### Basic Usage
Instead of using `git push`, use `git-guard push`:

```bash
# Instead of this:
git push origin main

# Use this:
git-guard push origin main
```

### What happens when you run it?

1. **If everything is clean** ✅
   ```
   🚀 git-guard: Checking staged files before push...
   📤 Target: origin -> main
   📋 Found 0 staged files
   ✅ All checks passed! Ready to push.
   ```

2. **If you have unwanted files** 🚫
   ```
   🚀 git-guard: Checking staged files before push...
   📤 Target: origin -> main
   📋 Found 2 staged files
   📁 Staged files:
      - src/main.rs
      - .env
   
   🚫 Push blocked! Found 1 blocked file(s):
      ❌ .env (blocked by pattern: .env)
   
   💡 To fix this:
      1. Remove blocked files: git rm --cached <file>
      2. Add to .gitignore to prevent future staging
      3. Commit the changes
   ```

---

## 🧪 Try It Out

### Test the Installation
```bash
# Check if git-guard is installed
git-guard --version

# See all available commands
git-guard --help

# See push command help
git-guard push --help
```

### Test in Your Project
```bash
# Go to any git repository
cd your-project

# Try a normal push (should work if clean)
git-guard push origin main

# Test with a blocked file
echo "secret=password123" > .env
git add .env
git-guard push origin main  # This will be blocked!
```

### Test HTTPS Warning
If your remote uses HTTPS, you'll see a security warning:
```bash
git remote -v  # Check your remote URL
git-guard push origin main  # Will warn about HTTPS
```

### Example Output
When you have blocked files, you'll see:
```
🚀 git-guard: Checking staged files before push...
📤 Target: origin -> main
📋 Found 1 staged files
📁 Staged files:
   - .env

🚫 Push blocked! Found 1 blocked file(s):
   ❌ .env (blocked by pattern: .env)

💡 To fix this:
   1. Remove blocked files: git rm --cached <file>
   2. Add to .gitignore to prevent future staging
   3. Commit the changes
```

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

## 📋 Current Status

### ✅ Implemented Features
- **Git Repository Detection** (`git.rs`) — Checks if current directory is a git repository
- **Staged File Analysis** (`git.rs`, used in `main.rs`) — Lists all staged files before pushing
- **File Blocking Rules** (`rules.rs`, used in `main.rs`) — Pattern matching for blocked files
- **HTTPS Remote Detection** (`git.rs`, used in `main.rs`) — Identifies and warns about HTTPS remotes
- **CLI Interface** (`main.rs`) — Clean command-line interface with help and version info

### 🔄 In Development
- **Git Push Execution** — Actual git push command execution
- **Configuration Support** — User-defined blocking rules

---

## ⚠️ HTTPS Remote Warning

If you're using a Git remote like:

```
https://github.com/user/repo.git
```

You’ll get a warning:

```
⚠️  You're using an HTTPS remote: https://github.com/user/repo.git
👉 It's recommended to use SSH for pushing to Git remotes.
   Example: git@github.com:user/repo.git
   See: https://docs.github.com/en/authentication/connecting-to-github-with-ssh
```

Why switch? SSH is more secure and avoids repeated password/token prompts.

---

## 🔁 Optional: Git Wrapper (Recommended)

You can alias `git push` to run `git-guard` instead:

### Step 1: Create a wrapper script

Create a file `~/bin/git-wrapper`:

```bash
#!/bin/bash
if [[ "$1" == "push" ]]; then
  shift
  git-guard push "$@"
else
  command git "$@"
fi
```

Make it executable:

```bash
chmod +x ~/bin/git-wrapper
```

### Step 2: Alias `git` to your wrapper

In your `.bashrc` / `.zshrc`:

```bash
alias git='git-wrapper'
```

Reload your shell:

```bash
source ~/.bashrc  # or ~/.zshrc
```

Now, `git push` automatically runs `git-guard`.

---

## 👷‍♂️ Contributing

Want to add `.gitignore` support or config files? PRs welcome!

---

## 📜 License

MIT

---

## 💡 Author

Made by [Nishant](https://github.com/nishujangra)