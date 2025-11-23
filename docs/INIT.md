# xg Project Creation Guide

xg provides project generation capabilities for quickly scaffolding new applications with production-ready structure and configurations.

## Quick Start

### Basic Usage
```bash
# Create a Go API server
xg create-go-app --project "my-api" --rest-framework gin
```

This creates a complete Go project structure with:
- Idiomatic folder layout (`cmd/`, `config/`, `internal/`)
- Environment-specific configuration files
- Framework bootstrap code
- Go module initialization and dependency management

## Command Syntax

```bash
xg create-go-app --project <NAME> --rest-framework <FRAMEWORK>
```

### Required Parameters
- `--project`: Project name (used for directory and package names)
- `--rest-framework`: REST framework to use (`gin` or `echo`)

### Examples
```bash
# Create a Gin-based API
xg create-go-app --project "user-service" --rest-framework gin

# Create an Echo-based API
xg create-go-app --project "auth-api" --rest-framework echo
```

## Supported Frameworks

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

### Echo Framework
Best for: High-performance REST APIs, microservices

Features:
- Echo v4 with middleware
- Structured logging with Logrus
- Graceful shutdown
- CORS, security, and recovery middleware
- Request validation with go-playground/validator
- Database integration ready (GORM optional)

## Project Structure

Generated Go projects include:

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

## Features

### Configuration Management
- Environment-specific JSON configuration files
- Configuration validation and defaults
- Support for development/production environments
- Environment variable integration

### Code Generation
- Framework-specific main.go with proper setup
- Configuration loading with error handling
- Basic middleware and routing structure
- Database connection setup (when applicable)

### Build Setup
- Go module initialization
- Dependency management with `go mod tidy`
- Cross-platform build support

## Examples

### Complete Workflow

#### 1. Gin-based API
```bash
$ xg create-go-app --project "user-api" --rest-framework gin

# Navigate to project
$ cd user-api

# Install dependencies
$ go mod tidy

# Run the application
$ go run cmd/main.go
```

#### 2. Echo-based Microservice
```bash
$ xg create-go-app --project "notification-service" --rest-framework echo

# Navigate to project
$ cd notification-service

# Install dependencies
$ go mod tidy

# Run the application
$ go run cmd/main.go
```

## Customization

### Modifying Generated Code
Generated projects provide a solid foundation that you can customize:

- Add routes and handlers in the main.go file
- Modify configuration files for your environment
- Add middleware for authentication, logging, etc.
- Integrate with databases, caches, or other services

### Environment Configuration
Update the JSON configuration files for your specific needs:

```json
{
  "environment": "development",
  "server": {
    "host": "localhost",
    "port": 8000
  }
}
```

## Next Steps

After project creation:

1. Review the generated code structure
2. Install dependencies with `go mod tidy`
3. Run the application with `go run cmd/main.go`
4. Add your business logic and routes
5. Configure your database connections
6. Set up testing and CI/CD pipelines
7. Deploy to your preferred hosting platform

## Contributing

To add new frameworks or improve existing templates:

1. Add generator logic to `src/generators/go/`
2. Update CLI argument parsing in `src/cli/`
3. Test the new framework generation
4. Submit a pull request