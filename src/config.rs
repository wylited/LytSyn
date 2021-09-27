use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    gauge: [i16; 3],
    borders: [i16; 3],
    minor_text: [i16; 3],
    major_text: [i16; 3],
    selectsymbol: String,
}

impl Theme {
    pub fn default() -> Theme {
        Theme {
            gauge: [85, 170, 200],
            borders: [30, 30, 60],
            minor_text: [255, 255, 255],
            major_text: [255, 200, 70],
            selectsymbol: ">>".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    discord: bool,
    server: bool,
    caching: bool,
    volume: i8,
}


impl Default for Settings {
    fn default() -> Self {
        Self {
            discord: true,
            server: true,
            caching: false,
            volume: 95,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Servers {
    servers: Vec<Server>,
}

impl Servers {
    pub fn default() -> Self {
        Self {
            servers: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    hostname: String,
    port: u16,
    ip4: [i8; 4],
}

impl Server {
    pub fn new(hostname: String, port: u16, ip4: [i8; 4]) -> Self {
        Self {
            hostname,
            port,
            ip4,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MuConfig {
    rmu: i32,
    theme: Theme,
    settings: Settings,
    servers: Servers,
}

impl MuConfig {
    pub fn default() -> Self {
        Self {  
            rmu: 12345678,
            theme: Theme::default(),
            settings: Settings::default(),
            servers: Servers::default(),
        }
    }

    pub fn get() -> MuConfig {
        let config_file: PathBuf = ProjectDirs::from("io", "Wylited",  "RustMU").unwrap().config_dir().to_path_buf().join("config.yaml");
        
        let config_string = fs::read_to_string(&config_file).unwrap_or("".to_string());
        let config: MuConfig = serde_yaml::Deserializer::from_str(&config_string).unwrap_or_else(
            MuConfig::default()
        );
        config
    }
}