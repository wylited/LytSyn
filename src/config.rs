use std::{ path::PathBuf, fs::OpenOptions, fs};
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
            borders: [66, 138, 138],
            minor_text: [140, 200, 200],
            major_text: [100, 180, 180],
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
    pub dark: bool,
}


impl Default for Settings {
    fn default() -> Self {
        Self {
            discord: true,
            server: true,
            caching: false,
            volume: 95,
            dark: true,
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
pub struct LytConfig {
    pub rmu: i32,
    pub theme: Theme,
    pub settings: Settings,
    pub servers: Servers,
}

impl LytConfig {
    pub fn default() -> Self {
        Self {  
            rmu: 12345678,
            theme: Theme::default(),
            settings: Settings::default(),
            servers: Servers::default(),
        }
    }

    pub fn get() -> LytConfig {
        let config_file: PathBuf = ProjectDirs::from("io", "Wylited",  "LytSyn").unwrap().config_dir().to_path_buf();
        let f = config_file.join("config.toml");
        let _res = fs::create_dir_all(config_file);
        let _foo = OpenOptions::new().read(true).open(&f); 
        let config_string = fs::read_to_string(&f);

        let newconfig = || -> LytConfig{
            let toml_string = toml::to_string(&LytConfig::default()).expect("Could not encode TOML value");
            fs::write(f, toml_string).expect("Could not write to file!");
            LytConfig::default()
        };
        
        let config: LytConfig = match config_string {
            Ok(file) => toml::from_str(&file).unwrap(),
            Err(_) => newconfig(),
        };

        config
    }
}