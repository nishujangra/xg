use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use crate::commands;
use crate::generators::go::config::{Config, Server};
use crate::utils::validation::is_project_name_valid;
use crate::writters::{json_writer, text_writer};


pub fn init(project: &str) {
    if !is_project_name_valid(project) {
        eprintln!("Invalid project name '{}'", project);
        return
    }

    println!("Creating {} directory!!!", project);

    match fs::create_dir(project) {
        Ok(_) => {
            println!("{} Directory created successfully", project);
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {
                    eprintln!("Directory '{}' already exists", project);
                },
                ErrorKind::PermissionDenied => {
                    eprint!("Permission denied while creating '{}'", project)
                },
                _ => {
                    eprintln!("Unexpected error: {}", e);
                }
            }
        }
    };

    // create folder structure
    // cmd/, config/, internal/config
    let cmd_path = Path::new(project).join("cmd");
    match fs::create_dir(&cmd_path) {
        Ok(_) => println!("Created {:?}", cmd_path),
        Err(e) => eprintln!("Error: {}", e),
    };

    let config_path = Path::new(project).join("config");
    match fs::create_dir(&config_path) {
        Ok(_) => println!("Created {:?}", config_path),
        Err(e) => eprintln!("Error: {}", e),
    };
    
    let internal_config_path = Path::new(project).join("internal").join("config");
    match fs::create_dir_all(&internal_config_path) {
        Ok(_) => println!("Created {:?}", internal_config_path),
        Err(e) => eprintln!("Error: {}", e),
    };

    // config/config.development.json, config/config.production.json sample configs
    let config_dev = Config {
        environment: "development".into(),
        server: Server {
            host: "localhost".into(),
            port: 8000,
        },
    };

    json_writer::write_json_file(
        &format!("{}/config/config.development.json", project),
        &config_dev
    ).unwrap();

    let config_prod = Config {
        environment: "production".into(),
        server: Server {
            host: "localhost".into(),
            port: 8000,
        },
    };

    json_writer::write_json_file(
        &format!("{}/config/config.production.json", project),
        &config_prod
    ).unwrap();

    // cmd/main.go -> start a sample gin server with ping route
    let main_go = r#"
package main

import (
    "log"
	"github.com/labstack/echo/v4"
)

func main() {
    cfg, err := config.Load()
    if err != nil {
		log.Fatalf("Failed to load configuration: %v", err)
	}

    // Echo Router
    e := echo.New()

    if cfg.Environment == "development" {
		e.Debug = true
	}

    // Run Router
    e.Logger.Fatal(e.Start(cfg.GetServerAddress()))
}
    "#;

    text_writer::write_text_file(
        &format!("{}/cmd/main.go", project),
        main_go
    ).unwrap();

    // internal/config/config.go -> reading config
    let internal_config_config_go = r#"
package config

import (
	"encoding/json"
	"fmt"
	"os"
)

type Config struct {
	Environment string         `json:"environment"`
	Server      ServerConfig   `json:"server"`
}

type ServerConfig struct {
	Host string `json:"host"`
	Port int    `json:"port"`
}

func Load() (*Config, error) {
	// Get environment from ENV var, default to development
	env := os.Getenv("ENV")
	if env == "" {
		env = "development"
	}

	// Build config file path
	configPath := fmt.Sprintf("config/config.%s.json", env)

	// Check if config file exists
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		return nil, fmt.Errorf("configuration file not found: %s", configPath)
	}

	// Read config file
	data, err := os.ReadFile(configPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read config file %s: %w", configPath, err)
	}

	// Parse JSON
	var config Config
	if err := json.Unmarshal(data, &config); err != nil {
		return nil, fmt.Errorf("failed to parse config file %s: %w", configPath, err)
	}

	// Validate configuration
	if err := validateConfig(&config); err != nil {
		return nil, fmt.Errorf("invalid configuration: %w", err)
	}

	return &config, nil
}

func validateConfig(config *Config) error {
	if config.Environment == "" {
		// default to development if environment is not set
		config.Environment = "development"
	}

	if config.Server.Port <= 0 || config.Server.Port > 65535 {
		// default to 8080 if port is not set
		config.Server.Port = 8080
	}

	if config.Server.Host == "" {
		// default to localhost if host is not set
		config.Server.Host = "localhost"
	}
}

func (c *Config) GetServerAddress() string {
	return fmt.Sprintf("%s:%d", c.Server.Host, c.Server.Port)
}

// IsDevelopment returns true if running in development environment
func (c *Config) IsDevelopment() bool {
	return c.Environment == "development"
}

// IsProduction returns true if running in production environment
func (c *Config) IsProduction() bool {
	return c.Environment == "production"
}
    "#;

    text_writer::write_text_file(
        &format!("{}/internal/config/config.go", project),
        internal_config_config_go
    ).unwrap();

    // go mod init  -> run cli command
    commands::go_cmd::run_go_mod_init(project).unwrap();

    // go mod tidy
    commands::go_cmd::run_go_mod_tidy(project).unwrap();

}