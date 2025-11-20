use serde::Serialize;

#[derive(Serialize)]
pub struct Config {
    pub environment: String,
    pub server: Server,
}

#[derive(Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}