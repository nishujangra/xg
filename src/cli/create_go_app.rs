use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone)]
pub struct GoArgs {
    /// Project name
    #[arg(short = 'p', long = "project")]
    pub project: String,

    /// REST framework to use (gin/echo)
    #[arg(short = 'r', long = "rest-framework", value_enum)]
    pub rest_framework: RestFramework,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum RestFramework {
    Gin,
    Echo,
}
