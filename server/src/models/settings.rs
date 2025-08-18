use yaml_rust::YamlLoader;
use std::fs;

pub struct Settings {
    pub appid: String,
    pub secret: String,
    pub image_dir: String,
    pub server_scheme: String,
    pub server_host: String,
    pub server_port: String,
}

impl Settings {
    pub fn new(yaml_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_content = fs::read_to_string(yaml_path)?;
        let docs = YamlLoader::load_from_str(&yaml_content)?;
        let doc = &docs[0]; // 获取第一个 YAML 文档

        Ok(Settings {
            appid: doc["appid"].as_str().unwrap_or_default().to_string(),
            secret: doc["secret"].as_str().unwrap_or_default().to_string(),
            image_dir: doc["image_dir"].as_str().unwrap_or_default().to_string(),
            server_scheme: doc["server_scheme"].as_str().unwrap_or("https").to_string(),
            server_host: doc["server_host"].as_str().unwrap_or("localhost").to_string(),
            server_port: doc["server_port"].as_str().unwrap_or("").to_string(),
        })
    }

    pub fn server_url(&self) -> String {
        if !self.server_port.is_empty() {
            format!("{}://{}:{}", self.server_scheme, self.server_host, self.server_port)
        } else {
            format!("{}://{}", self.server_scheme, self.server_host)
        }
    }
}