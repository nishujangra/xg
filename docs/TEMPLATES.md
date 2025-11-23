# xg Template Reference

This document provides information about xg's project generation capabilities and supported frameworks.

## Template System Overview

xg templates are code generators that create complete project structures with production-ready configurations. Currently, xg supports Go project generation with framework-specific templates.

## Go Templates

### Gin Framework
Best for: Full-featured web applications, REST APIs with authentication

Features:
- Gin web framework with extensive middleware
- JWT authentication ready
- Rate limiting and CORS
- Structured logging
- Database ORM integration (GORM)
- Testing framework setup
- API documentation with Swagger

Generated Structure:
```
my-api/
├── cmd/
│   └── main.go                 # Application entry point
├── config/
│   ├── config.development.json # Development configuration
│   └── config.production.json  # Production configuration
├── internal/
│   └── config/
│       └── config.go           # Configuration loading and validation
└── go.mod                      # Go module file
```

Key Dependencies:
```go
github.com/gin-gonic/gin
```

### Echo Framework
Best for: High-performance REST APIs, microservices

Features:
- Echo v4 with middleware
- Structured logging with Logrus
- Graceful shutdown
- CORS, security, and recovery middleware
- Request validation with go-playground/validator
- Database integration ready (GORM optional)

Generated Structure:
```
my-api/
├── cmd/
│   └── main.go                 # Application entry point
├── config/
│   ├── config.development.json # Development configuration
│   └── config.production.json  # Production configuration
├── internal/
│   └── config/
│       └── config.go           # Configuration loading and validation
└── go.mod                      # Go module file
```

Key Dependencies:
```go
github.com/labstack/echo/v4
```

## Template Features

### Common Features Across All Templates

#### Configuration Management
- Environment-specific JSON configuration files
- Configuration validation and defaults
- Support for development/production environments
- Environment variable integration

#### Code Generation
- Framework-specific main.go with proper setup
- Configuration loading with error handling
- Basic middleware and routing structure

#### Build Setup
- Go module initialization
- Dependency management with `go mod tidy`
- Cross-platform build support

### Go-Specific Features
- Go modules (`go.mod`)
- Dependency management
- Build optimization
- Cross-compilation support

## Template Usage

### Creating Projects
```bash
# Generate a Gin-based API
xg create-go-app --project "user-api" --rest-framework gin

# Generate an Echo-based API
xg create-go-app --project "notification-service" --rest-framework echo
```

### Project Structure Details

#### cmd/main.go
The main application entry point with:
- Framework initialization
- Configuration loading
- Basic middleware setup
- Server startup

#### config/ Directory
Contains environment-specific configuration:
- `config.development.json` - Development settings
- `config.production.json` - Production settings

#### internal/config/config.go
Configuration management with:
- JSON file loading
- Environment variable support
- Validation and defaults
- Environment detection

## Customization

### Modifying Generated Projects
Generated projects provide a foundation that you can customize:

- Add routes and handlers in main.go
- Modify configuration files for your environment
- Add middleware for authentication, logging, etc.
- Integrate with databases, caches, or other services

### Environment Configuration
Update JSON configuration files for your specific needs:

```json
{
  "environment": "development",
  "server": {
    "host": "localhost",
    "port": 8000
  }
}
```

## Contributing Templates

To add new frameworks or improve existing templates:

1. Add generator logic to `src/generators/go/`
2. Update CLI argument parsing in `src/cli/`
3. Test the new framework generation
4. Submit a pull request

Template requirements:
- Well-documented code
- Error handling
- Security best practices
- Performance considerations

## Future Template Support

Planned language and framework expansions:
- Additional Go frameworks
- JavaScript/TypeScript support
- Python frameworks
- Rust templates
- And more based on community demand