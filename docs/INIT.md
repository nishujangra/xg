# 🚀 xg init - Project Creation Guide

`xg init` is xg's powerful project templating system that helps you create new projects instantly with the right structure, dependencies, and configurations.

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Command Syntax](#-command-syntax)
- [Interactive Framework Selection](#-interactive-framework-selection)
- [Supported Languages & Frameworks](#-supported-languages--frameworks)
- [Project Structure](#-project-structure)
- [Customization Options](#-customization-options)
- [Examples](#-examples)
- [Advanced Usage](#-advanced-usage)
- [Troubleshooting](#-troubleshooting)

---

## 🎯 Quick Start

### Basic Usage
```bash
# Create a Go API server
xg init -lang golang -name "my-api"

# Create a React application
xg init -lang javascript -name "my-app"

# Create a Rust CLI tool
xg init -lang rust -name "my-tool"
```

### What Happens?
1. **Directory Creation**: Creates `./my-project/` directory
2. **Framework Selection**: Interactive prompts guide your choices
3. **File Generation**: Creates all necessary files and directories
4. **Git Initialization**: Sets up git repository with initial commit
5. **Dependency Setup**: Generates package files, Docker configs, etc.

---

## 📝 Command Syntax

```bash
xg init [OPTIONS]

OPTIONS:
    -l, --lang <LANGUAGE>    Programming language (required)
    -n, --name <NAME>        Project name (required)
    -f, --framework <FRAMEWORK>  Skip interactive prompt (optional)
    -t, --template <TEMPLATE>    Use specific template (optional)
    --no-git                  Don't initialize git repository
    --no-deps                 Don't generate dependency files
    -v, --verbose             Show detailed output
    -h, --help                Show help message
```

### Required Parameters
- **`-l, --lang`**: Programming language (golang, javascript, rust, python, etc.)
- **`-n, --name`**: Project name (used for directory and package names)

### Optional Parameters
- **`-f, --framework`**: Pre-select framework to skip interactive prompts
- **`-t, --template`**: Use specific template variant
- **`--no-git`**: Skip git repository initialization
- **`--no-deps`**: Skip dependency file generation

---

## 🎪 Interactive Framework Selection

xg uses intelligent prompts to help you choose the perfect stack for your project.

### Example Session: Go API Server
```bash
$ xg init -lang golang -name "user-api"

🚀 Creating new Go project: user-api
📁 Directory: ./user-api/

🎯 Which framework would you like to use for your Go API server?

1. Echo   - High performance, minimalist Go web framework
2. Gin    - The fastest full-featured web framework for Go
3. Fiber  - Express-inspired web framework built on Fasthttp
4. Chi    - Lightweight, idiomatic and composable router
5. Standard Library - Go's net/http with middleware

Enter your choice (1-5) [default: 2]: 2

✅ Selected: Gin
🔧 Generating project structure...
📦 Creating go.mod, main.go, handlers, middleware...
🐳 Adding Dockerfile and docker-compose.yml...
📝 Setting up README.md and .gitignore...
🔐 Creating .env.example...
🌱 Initializing git repository...
🎉 Project 'user-api' created successfully!

Next steps:
  cd user-api
  go mod tidy
  go run main.go

Happy coding! 🚀
```

### Smart Defaults
- **Go**: Gin (most popular and feature-rich)
- **JavaScript**: React with Vite (modern and fast)
- **Rust**: CLI tool (most common use case)
- **Python**: FastAPI (async, modern, well-documented)

---

## 🌍 Supported Languages & Frameworks

### Go (golang)
| Framework | Description | Use Case |
|-----------|-------------|----------|
| **Echo** | High performance, minimalist | REST APIs, microservices |
| **Gin** | Full-featured, middleware-rich | Web apps, APIs with auth |
| **Fiber** | Express-inspired, very fast | High-performance APIs |
| **Chi** | Lightweight, composable | Simple APIs, learning |
| **Std Lib** | Go's net/http + middleware | Minimal dependencies |

### JavaScript/TypeScript
| Framework | Type | Description |
|-----------|------|-------------|
| **React** | JS/TS | Component-based UI library |
| **Vue** | JS/TS | Progressive framework |
| **Svelte** | JS/TS | Compile-time framework |
| **Node.js** | JS/TS | Server-side JavaScript |
| **Express** | JS/TS | Web application framework |
| **Fastify** | JS/TS | Fast web framework |

### Rust
| Template | Description |
|----------|-------------|
| **CLI Tool** | Command-line application |
| **Library** | Reusable Rust crate |
| **Web App** | Web application with Axum |
| **API Server** | REST API with Actix |
| **Async Runtime** | Tokio-based application |

### Python
| Framework | Description |
|-----------|-------------|
| **FastAPI** | Modern async API framework |
| **Flask** | Lightweight web framework |
| **Django** | Full-featured web framework |
| **Data Science** | Jupyter + scientific stack |
| **CLI Tool** | Command-line application |

---

## 🏗️ Project Structure

### Go API Server (Gin)
```
user-api/
├── cmd/
│   └── server/
│       └── main.go          # Application entry point
├── internal/
│   ├── handlers/            # HTTP handlers
│   ├── middleware/          # Custom middleware
│   └── models/              # Data models
├── pkg/                     # Public packages
├── configs/                 # Configuration files
├── scripts/                 # Build/deployment scripts
├── Dockerfile               # Container definition
├── docker-compose.yml       # Local development setup
├── go.mod                   # Go modules
├── Makefile                 # Build automation
├── .env.example             # Environment variables template
├── .gitignore               # Git ignore rules
└── README.md                # Project documentation
```

### React Application
```
react-app/
├── public/                  # Static assets
├── src/
│   ├── components/          # React components
│   ├── hooks/               # Custom hooks
│   ├── utils/               # Utility functions
│   ├── App.jsx              # Main app component
│   └── main.jsx             # App entry point
├── .gitignore               # Git ignore rules
├── index.html               # HTML template
├── package.json             # Dependencies and scripts
├── vite.config.js           # Vite configuration
├── tailwind.config.js       # Tailwind CSS config
└── README.md                # Project documentation
```

### Rust CLI Tool
```
cli-tool/
├── src/
│   └── main.rs              # Main application
├── Cargo.toml               # Package configuration
├── .gitignore               # Git ignore rules
├── README.md                # Project documentation
└── LICENSE                  # MIT license
```

---

## 🎨 Customization Options

### Framework-Specific Options
```bash
# Pre-select framework to skip prompts
xg init -lang golang -name "api" -f gin

# Use specific template variant
xg init -lang javascript -name "app" -t "react-typescript"
```

### Advanced Configuration
```bash
# Skip git initialization
xg init -lang rust -name "tool" --no-git

# Skip dependency files (just structure)
xg init -lang python -name "app" --no-deps

# Verbose output for debugging
xg init -lang golang -name "api" -v
```

### Custom Templates
xg supports community templates and custom configurations:

```bash
# Use community template
xg init -lang golang -t "microservice-gin"

# Use organization template
xg init -lang javascript -t "company-react-setup"
```

---

## 📚 Examples

### Complete Workflow Examples

#### 1. Go Microservice
```bash
$ xg init -lang golang -name "user-service"
# Select: Gin
# Result: Complete microservice with Docker, tests, CI/CD

$ cd user-service
$ go mod tidy
$ docker-compose up -d
$ go test ./...
```

#### 2. React Dashboard
```bash
$ xg init -lang javascript -name "admin-dashboard"
# Select: React + TypeScript + Tailwind
# Result: Modern dashboard with routing, state management

$ cd admin-dashboard
$ npm install
$ npm run dev
```

#### 3. Python Data API
```bash
$ xg init -lang python -name "data-api"
# Select: FastAPI + SQLAlchemy
# Result: Async API with database models, migrations

$ cd data-api
$ pip install -r requirements.txt
$ uvicorn main:app --reload
```

#### 4. Rust CLI Tool
```bash
$ xg init -lang rust -name "file-organizer"
# Select: CLI Tool + Clap
# Result: Command-line tool with argument parsing

$ cd file-organizer
$ cargo build
$ cargo run -- --help
```

---

## 🔧 Advanced Usage

### Template Variables
xg templates support dynamic content based on your choices:

- `{{project_name}}` - Your project name
- `{{author}}` - Git author name
- `{{year}}` - Current year
- `{{framework}}` - Selected framework
- `{{language}}` - Programming language

### Custom Configuration Files
Create `.xg.toml` in your home directory for global preferences:

```toml
[defaults]
golang = "gin"
javascript = "react-typescript"
rust = "cli"
python = "fastapi"

[author]
name = "Your Name"
email = "your.email@example.com"
```

### Environment-Specific Setup
xg can create different configurations for development/production:

```bash
# Development-focused setup
xg init -lang golang -name "api" --env dev

# Production-ready setup
xg init -lang golang -name "api" --env prod
```

---

## 🛠️ Troubleshooting

### Common Issues

#### "Directory already exists"
```bash
Error: Directory 'my-project' already exists

Solution: Choose a different name or remove the existing directory
xg init -lang golang -name "my-project-v2"
```

#### "Language not supported"
```bash
Error: Language 'kotlin' is not supported yet

Solution: Check supported languages with:
xg init --help
```

#### "Template generation failed"
```bash
Error: Failed to generate template files

Solution:
1. Check disk space
2. Ensure write permissions
3. Try with --verbose flag for more details
xg init -lang golang -name "test" -v
```

### Getting Help
```bash
# Show all available languages
xg init --help

# Show framework options for a language
xg init -lang golang --help

# Verbose output for debugging
xg init -lang rust -name "debug" -v
```

### Manual Cleanup
If something goes wrong during project creation:

```bash
# Remove incomplete project
rm -rf my-project

# Clean up any temporary files
find . -name ".xg-*" -type f -delete
```

---

## 🎯 Best Practices

### 1. **Choose Appropriate Frameworks**
- **APIs**: Gin/FastAPI for speed, Django for full-featured
- **CLIs**: Rust/Go for performance, Python for rapid development
- **Web Apps**: React/Vue for SPAs, Svelte for smaller apps

### 2. **Use Consistent Naming**
```bash
# Good: descriptive and consistent
xg init -lang golang -name "user-auth-service"
xg init -lang javascript -name "admin-dashboard"

# Avoid: unclear or inconsistent
xg init -lang golang -name "project1"
xg init -lang javascript -name "my-cool-app"
```

### 3. **Review Generated Files**
Always check the generated files before committing:
```bash
cd my-new-project
xg status  # See what was created
cat README.md  # Check documentation
# Review configuration files
```

### 4. **Keep Templates Updated**
xg templates evolve with best practices:
```bash
# Check for template updates
xg init --check-updates
```

---

## 🚀 Next Steps

After creating your project:

1. **Explore the structure**: `tree .` or `ls -la`
2. **Install dependencies**: `go mod tidy`, `npm install`, `pip install -r requirements.txt`
3. **Run the project**: Check the README.md for commands
4. **Customize**: Modify generated files to fit your needs
5. **Commit**: `xg add . && xg commit -m "Initial commit"`

---

## 🤝 Contributing Templates

Want to add a new language or framework?

1. **Fork the repository**
2. **Add templates** in `templates/` directory
3. **Update documentation**
4. **Submit a pull request**

Template format:
```
templates/
├── golang/
│   ├── gin/
│   │   ├── template-files/
│   │   └── config.json
│   └── echo/
└── javascript/
    ├── react/
    └── vue/
```

---

## 📚 Related Documentation

- [Complete Usage Guide](USAGE.md) - All xg commands
- [Supported Templates](TEMPLATES.md) - Detailed template reference
- [Installation Guide](INSTALLATION.md) - Getting started
- [Contributing Guide](../CONTRIBUTING.md) - Help improve xg
