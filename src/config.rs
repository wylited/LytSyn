use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub gauge: [u8; 3],
    pub borders: [u8; 3],
    pub minor_text: [u8; 3],
    pub major_text: [u8; 3],
    pub selectsymbol: String,
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
    pub discord: bool,
    pub server: bool,
    pub caching: bool,
    pub volume: u8,
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
pub struct Server {
    pub hostname: String,
    pub port: u16,
    pub ip4: [u8; 4],
}

impl Server {
    pub fn new(hostname: String, port: u16, ip4: [u8; 4]) -> Self {
        Self {
            hostname,
            port,
            ip4,
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Servers {
    pub servers: Vec<Server>,
}

impl Servers {
    pub fn default() -> Self {
        Self {
            servers: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MuConfig {
    pub rmu: i32,
    pub theme: Theme,
    pub settings: Settings,
    pub servers: Servers,
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
        let config_file: PathBuf = ProjectDirs::from("io", "Wylited",  "RustMU").unwrap().config_dir().to_path_buf();
        let config_string = fs::read_to_string(&config_file.join("config.toml"));
        
        let config: MuConfig = match config_string {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => MuConfig::default(),
        };
        config
    }
}