pub struct ServerConfig {
    pub host: String, 
    pub port: u16,
    pub threads: usize,
}

pub fn get_server_config() -> ServerConfig {
    ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        threads: 4,
    }
}