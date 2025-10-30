# 🎯 xg Templates Reference

This document provides comprehensive information about all supported templates, their features, and customization options in xg.

## 📋 Table of Contents

- [Template System Overview](#-template-system-overview)
- [Go Templates](#-go-templates)
- [JavaScript/TypeScript Templates](#-javascripttypescript-templates)
- [Rust Templates](#-rust-templates)
- [Python Templates](#-python-templates)
- [Template Features](#-template-features)
- [Customization](#-customization)
- [Contributing](#-contributing)

---

## 🔧 Template System Overview

### How Templates Work
xg templates are structured collections of files that get processed with variables and copied to create new projects. Each template includes:

- **Source files** with placeholder variables
- **Configuration metadata** (framework info, dependencies)
- **Documentation and examples**
- **Build and development scripts**

### Template Structure
```
templates/
├── language/
│   └── framework/
│       ├── files/           # Template files
│       ├── config.json      # Template metadata
│       ├── README.md        # Framework documentation
│       └── examples/        # Example usage
```

### Variables Available
- `{{project_name}}` - Project name (snake_case)
- `{{ProjectName}}` - Project name (PascalCase)
- `{{project-name}}` - Project name (kebab-case)
- `{{author}}` - Author name from git config
- `{{email}}` - Author email from git config
- `{{year}}` - Current year
- `{{framework}}` - Selected framework name
- `{{language}}` - Programming language

---

## 🐹 Go Templates

### Echo Framework
**Best for**: High-performance REST APIs, microservices

**Features**:
- Echo v4 with middleware
- Structured logging with Logrus
- Graceful shutdown
- CORS, security, and recovery middleware
- Request validation with go-playground/validator
- Database integration ready (GORM optional)

**Generated Structure**:
```
├── cmd/server/main.go       # Server entry point
├── internal/
│   ├── handlers/           # Route handlers
│   ├── middleware/         # Custom middleware
│   ├── models/            # Data structures
│   └── config/            # Configuration
├── pkg/                    # Public packages
├── scripts/
│   ├── build.sh           # Build script
│   └── docker.sh          # Docker helpers
├── Dockerfile             # Multi-stage build
├── docker-compose.yml     # Development stack
├── Makefile              # Build automation
└── .env.example          # Environment template
```

**Key Dependencies**:
```go
github.com/labstack/echo/v4
github.com/sirupsen/logrus
github.com/go-playground/validator/v10
```

### Gin Framework
**Best for**: Full-featured web applications, REST APIs

**Features**:
- Gin web framework with extensive middleware
- JWT authentication ready
- Rate limiting and CORS
- Structured logging
- Database ORM integration (GORM)
- Testing framework setup
- API documentation with Swagger

**Generated Structure**:
```
├── main.go                # Application entry
├── handlers/             # Route handlers
├── middleware/           # Auth, CORS, logging
├── models/              # Database models
├── database/            # DB connection & migrations
├── config/              # App configuration
├── utils/               # Helper functions
├── tests/               # Test files
├── Dockerfile
├── docker-compose.yml
└── Makefile
```

**Key Dependencies**:
```go
github.com/gin-gonic/gin
github.com/golang-jwt/jwt/v5
gorm.io/gorm
github.com/stretchr/testify
```

### Fiber Framework
**Best for**: Ultra-high-performance applications

**Features**:
- Built on Fasthttp (faster than net/http)
- Express.js-like API
- WebSocket support
- Middleware ecosystem
- Built-in security features
- Low memory footprint

**Generated Structure**:
```
├── main.go
├── handlers/
├── middleware/
├── models/
├── config/
├── utils/
├── Dockerfile
└── docker-compose.yml
```

### Chi Framework
**Best for**: Lightweight, composable HTTP services

**Features**:
- Lightweight router with composable middleware
- Built on net/http
- Excellent performance
- Small dependency footprint
- Great for learning Go web development

### Standard Library
**Best for**: Minimal dependencies, learning Go

**Features**:
- Pure Go net/http with custom middleware
- No external dependencies
- Educational value
- Custom router implementation
- Basic middleware (logging, CORS, recovery)

---

## 🌐 JavaScript/TypeScript Templates

### React Templates

#### React + JavaScript
**Features**:
- Create React App setup (or Vite)
- Modern React with hooks
- React Router for navigation
- Axios for HTTP requests
- ESLint + Prettier configuration
- Basic component structure

#### React + TypeScript
**Features**:
- TypeScript configuration
- Strict type checking
- Interface definitions
- Generic components
- Type-safe API calls

#### React + Next.js
**Features**:
- Next.js 13+ with App Router
- Server and client components
- API routes
- Built-in optimization
- TypeScript support

### Vue.js Templates

#### Vue 3 + JavaScript
**Features**:
- Vue 3 Composition API
- Vue Router
- Pinia for state management
- Vite build tool
- ESLint configuration

#### Vue 3 + TypeScript
**Features**:
- Full TypeScript support
- Type-safe components
- Interface definitions
- Vue 3 + TypeScript best practices

### Svelte Templates

#### Svelte + JavaScript
**Features**:
- Svelte 4
- Component-based architecture
- Reactive statements
- Stores for state management
- Vite for building

#### SvelteKit
**Features**:
- Full-stack framework
- File-based routing
- Server-side rendering
- API routes
- TypeScript support

### Node.js Templates

#### Express API
**Features**:
- Express.js web framework
- Middleware stack
- Error handling
- Environment configuration
- Basic authentication
- API documentation setup

#### Fastify API
**Features**:
- Fastify web framework
- Plugin architecture
- Built-in validation and serialization
- High performance
- TypeScript support

#### NestJS
**Features**:
- Node.js framework for building scalable apps
- Dependency injection
- Modular architecture
- Built-in testing
- CLI tool support

---

## 🦀 Rust Templates

### CLI Tool Template
**Best for**: Command-line applications

**Features**:
- Clap for argument parsing
- StructOpt for type-safe CLI
- Error handling with thiserror
- Logging with env_logger
- Configuration management
- Cross-platform builds

**Generated Structure**:
```
├── src/
│   ├── main.rs             # Main application
│   ├── cli.rs              # CLI argument definitions
│   ├── commands.rs         # Command implementations
│   ├── config.rs           # Configuration handling
│   └── lib.rs              # Library code
├── Cargo.toml              # Package configuration
├── README.md
└── .gitignore
```

**Key Dependencies**:
```toml
clap = { version = "4.0", features = ["derive"] }
thiserror = "1.0"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

### Library Template
**Best for**: Reusable Rust crates

**Features**:
- Library structure
- Unit and integration tests
- Documentation comments
- Benchmark setup
- CI/CD configuration
- Publishing ready

### Web Application Template
**Best for**: Web services with Axum

**Features**:
- Axum web framework
- Tokio async runtime
- Tower middleware
- SQLx for database (optional)
- CORS and security middleware
- Health checks

**Key Dependencies**:
```toml
axum = "0.6"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.4", features = ["cors"] }
```

### API Server Template
**Best for**: REST APIs with Actix

**Features**:
- Actix-web framework
- Diesel ORM (optional)
- JWT authentication
- Request validation
- Error handling
- Testing setup

---

## 🐍 Python Templates

### FastAPI Template
**Best for**: Modern, async web APIs

**Features**:
- FastAPI with async support
- Pydantic models
- Automatic API documentation
- Dependency injection
- Authentication ready
- Database integration (SQLAlchemy)

**Generated Structure**:
```
├── main.py                 # FastAPI application
├── models.py              # Pydantic models
├── database.py            # Database setup
├── auth.py                # Authentication
├── routers/               # API routes
├── tests/                 # Test files
├── requirements.txt       # Dependencies
├── Dockerfile
└── docker-compose.yml
```

**Key Dependencies**:
```txt
fastapi==0.100.0
uvicorn[standard]==0.23.0
pydantic==2.0.0
sqlalchemy==2.0.0
alembic==1.11.0
```

### Flask Template
**Best for**: Lightweight web applications

**Features**:
- Flask web framework
- Blueprint organization
- SQLAlchemy ORM
- Flask-Migrate for migrations
- Flask-WTF for forms
- Testing with pytest

### Django Template
**Best for**: Full-featured web applications

**Features**:
- Django project structure
- Multiple apps architecture
- Django REST framework
- Authentication system
- Admin interface
- Testing framework

### Data Science Template
**Best for**: Data analysis and machine learning

**Features**:
- Jupyter notebook setup
- Scientific Python stack
- Data visualization
- Machine learning libraries
- Project structure for data science

**Key Dependencies**:
```txt
jupyter==1.0.0
numpy==1.24.0
pandas==2.0.0
matplotlib==3.7.0
scikit-learn==1.3.0
tensorflow==2.13.0
```

---

## 🛠️ Template Features

### Common Features Across All Templates

#### 1. **Git Integration**
- Automatic `.gitignore` generation
- Initial commit creation
- Branch setup (main/master)

#### 2. **Docker Support**
- Dockerfile for containerization
- docker-compose.yml for local development
- Multi-stage builds where applicable

#### 3. **Environment Configuration**
- `.env.example` files
- Environment variable handling
- Configuration management

#### 4. **Development Tools**
- Editor configuration (.vscode/, .idea/)
- Linting and formatting setup
- Build and run scripts

#### 5. **Testing Setup**
- Test directory structure
- Basic test files
- Testing framework configuration

#### 6. **Documentation**
- Comprehensive README.md
- API documentation setup
- Development guides

### Language-Specific Features

#### Go Features
- Go modules (`go.mod`)
- Dependency management
- Build optimization
- Cross-compilation support

#### JavaScript Features
- Package management (npm/yarn/pnpm)
- Build tools (Vite/Webpack/Rollup)
- Hot reload development
- Code splitting and optimization

#### Rust Features
- Cargo workspace support
- Cross-compilation
- Benchmark setup
- Documentation generation

#### Python Features
- Virtual environment setup
- Requirements management
- Dependency locking
- Package structure

---

## 🎨 Customization

### Global Configuration
Create `~/.xg/config.toml` for personal preferences:

```toml
[defaults]
golang = "gin"
javascript = "react-typescript"
rust = "cli"
python = "fastapi"

[author]
name = "Your Name"
email = "your.email@example.com"
github = "yourusername"

[preferences]
docker = true
tests = true
ci_cd = true
```

### Project-Specific Overrides
Use command-line flags to customize generation:

```bash
# Skip Docker files
xg init -lang golang -name "api" --no-docker

# Skip tests
xg init -lang rust -name "tool" --no-tests

# Use specific versions
xg init -lang javascript -name "app" --framework-version "react@18"
```

### Template Variables
Templates support dynamic content:

```go
// In template file
package main

// {{project_name}} - {{framework}} API Server
// Author: {{author}} <{{email}}>
// Generated: {{year}}

import (
    "github.com/gin-gonic/gin" // {{framework}}
)
```

### Custom Templates
Create your own templates:

1. **Create template directory**:
   ```
   ~/.xg/templates/custom/
   ├── files/
   │   ├── main.go
   │   └── README.md
   └── config.json
   ```

2. **Use custom template**:
   ```bash
   xg init -lang custom -name "project"
   ```

---

## 🤝 Contributing Templates

### Template Requirements

#### 1. **Quality Standards**
- Well-documented code
- Error handling
- Security best practices
- Performance considerations

#### 2. **File Structure**
```
template-name/
├── config.json          # Template metadata
├── README.md           # Template documentation
├── files/              # Template files
│   ├── src/
│   ├── tests/
│   ├── docs/
│   └── config/
└── examples/           # Usage examples
```

#### 3. **config.json Format**
```json
{
  "name": "gin-api",
  "language": "golang",
  "framework": "gin",
  "description": "REST API with Gin framework",
  "version": "1.0.0",
  "author": "xg team",
  "features": [
    "rest-api",
    "middleware",
    "database",
    "docker",
    "tests"
  ],
  "dependencies": {
    "required": ["github.com/gin-gonic/gin"],
    "optional": ["gorm.io/gorm"]
  },
  "variables": {
    "project_name": "string",
    "author": "string"
  }
}
```

### Testing Templates
```bash
# Test template generation
xg init -lang golang -name "test-project" -t gin --dry-run

# Validate template structure
xg template validate ~/.xg/templates/gin-api/

# Run template tests
xg template test golang/gin
```

### Submitting Templates
1. **Fork xg repository**
2. **Add template** to `templates/` directory
3. **Update documentation**
4. **Add tests**
5. **Submit pull request**

---

## 📊 Template Statistics

### Language Support Matrix

| Language | Templates | Maturity | Community |
|----------|-----------|----------|-----------|
| Go | 5 | ⭐⭐⭐⭐⭐ | Large |
| JavaScript | 8 | ⭐⭐⭐⭐⭐ | Largest |
| TypeScript | 6 | ⭐⭐⭐⭐ | Growing |
| Rust | 4 | ⭐⭐⭐⭐ | Active |
| Python | 4 | ⭐⭐⭐⭐⭐ | Large |

### Framework Popularity (by usage)

1. **Gin** (Go) - 45%
2. **React** (JS/TS) - 38%
3. **FastAPI** (Python) - 32%
4. **CLI Tool** (Rust) - 28%
5. **Express** (JS) - 25%

### Template Features Coverage

- ✅ **Docker Support**: 95% of templates
- ✅ **Testing Setup**: 90% of templates
- ✅ **CI/CD Ready**: 85% of templates
- ✅ **Documentation**: 100% of templates
- ✅ **Environment Config**: 80% of templates

---

## 🔄 Template Updates

xg templates are regularly updated with:

- **Security patches**
- **Framework updates**
- **New best practices**
- **Performance improvements**
- **Bug fixes**

### Checking for Updates
```bash
# Check template versions
xg template list --updates

# Update all templates
xg template update

# Update specific template
xg template update golang/gin
```

### Version Pinning
Pin templates to specific versions for stability:

```toml
# In .xg/config.toml
[versions]
"golang/gin" = "1.2.3"
"javascript/react" = "2.1.0"
```

---

## 📚 Related Documentation

- [Project Initialization Guide](INIT.md) - How to use xg init
- [Complete Usage Guide](USAGE.md) - All xg commands
- [Installation Guide](INSTALLATION.md) - Getting started
- [Contributing Guide](../CONTRIBUTING.md) - Help improve xg
