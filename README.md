# 🛡️ git-guard

A lightweight Rust CLI tool to block accidental `git push` of unwanted files like `.idea/`, `node_modules/`, `.env`, and more.

Stop embarrassing commits before they leave your machine.

---

## 🚀 Features

- Blocks `git push` if disallowed files are staged
- Prevents pushing editor configs, build artifacts, or sensitive files
- Warns users if using HTTPS remotes (recommends switching to SSH)
- Seamless passthrough to real `git push` if everything is clean
- Written in pure Rust — fast and dependency-free (except `regex`)

---

## 🧱 Installation

### 🔧 Prerequisites
- [Rust installed](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

### 📥 Install via Git

```bash
git clone https://github.com/nishujangra/git-guard.git
cd git-guard
cargo install --path .
```

This builds the binary and installs it to `$HOME/.cargo/bin/git-guard`.

---

## 🧪 Usage

```bash
# Regular usage
git add .
git commit -m "message"

# Run git-guard instead of git push
git-guard push origin main
```

---

## 🔒 What It Blocks

The push will be blocked if any of these are staged:

* `.idea/`
* `node_modules/`
* `target/`
* `.env`
* `.DS_Store`

> ℹ️ You’ll see a clear message if push is blocked.

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