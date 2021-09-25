use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use directories::ProjectDirs;

/*
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::fs;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};
use toml::Value;
use std::path::Path;


#[derive(Deserialize, Serialize, Debug)]
pub struct Theme {
    bar: [i8; 3],
    border: [i8; 3],
    minor_text: [i8; 3],
    major_text: [i8; 3],
    selectsymbol: String,
    foldersymbol: String,
}

impl Theme {
    pub fn new(
        barcolor: [i8; 3],
        bordercolor: [i8; 3],
        majorcolor: [i8; 3],
        minorcolor: [i8; 3],
        selectedchar: String,
        folderchar: String,
    ) -> Self {
        Self {
            bar: barcolor,
            border: bordercolor,
            minor_text: minorcolor,
            major_text: majorcolor,
            selectsymbol: selectedchar,
            foldersymbol: folderchar,
        }
    }
    pub fn default() -> Self {
        Self {
            bar: [191, 239, 255],
            border: [191, 239, 255],
            minor_text: [191, 239, 255],
            major_text: [191, 239, 255],
            selectsymbol: ">>".to_string(),
            foldersymbol: "|".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    discord: bool,
    network: bool,
    caching: bool,
    volume: i16,
}

impl Settings {
    pub fn new(ndiscord: bool, nnetwork: bool, ncaching: bool, nvolume: i16) -> Self {
        Self {
            discord: ndiscord,
            network: nnetwork,
            caching: ncaching,
            volume: nvolume,
        }
    }
    pub fn default() -> Self {
        Self {
            discord: true,
            network: false,
            caching: true,
            volume: 100,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CachedFile {}

impl CachedFile{

    pub fn new() -> Self {
        Self {

        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    owner: String,
    name: String,
    ip: Ipv4Addr,
    port: i32,
}

impl Server{
    pub fn new(nowner: String, nname: String, nip: Ipv4Addr, nport: i32) -> Self {
        Self {
            owner: nowner,
            name: nname,
            ip: nip,
            port: nport,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RustMUInst {
    version: String,
    UUID: String,
    servers: Vec<Server>,
    files: Vec<CachedFile>,
    setting: Settings,
    theme: Theme,
    configdir: PathBuf,
}

impl RustMUInst {
    pub fn new(ver: String, unique: String, server: Vec<Server>, file: Vec<CachedFile>, settings: Settings, themes: Theme, configpath: PathBuf) -> Self {
        Self {
            version: ver,
            UUID: unique,
            servers: server,
            files: file,
            setting: settings,
            theme: themes,
            configdir: configpath,
        }
    }

    pub fn get() -> RustMUInst {
        let config_path: &Path = ProjectDirs::from("io", "wylited", "RustMU").unwrap().config_dir();
        let config_file: PathBuf = config_path.join("config.toml");
        if !(config_file.exists()){
            let config = RustMUInst {
                version: "1.0.0".to_string(),
                UUID: "eba50a90-72e4-44d2-b8db-db1bafcc5d15".to_string(),
                servers: Vec::new(),
                files: Vec::new(),
                setting: Settings::default(),
                theme: Theme::default(),
                configdir: ProjectDirs::from("io", "wylited", "RustMU").unwrap().config_dir().to_path_buf(),
            };

            std::fs::write(config_path.join("config.toml"), toml::to_string(&config).unwrap()).expect("could not write to file");
            return config;
        } else {
            let config: RustMUInst = toml::from_str(fs::read_to_string(config_file)).parse();
            return config;
        }
    }
}
*/

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
        if !(config_file.exists()){
            let yamlstr = r#"---
rMU: "uuid"

theme:
  gauge_color: [255, 255, 255]
  border: [255, 255, 255]
  minor_text: [255, 255, 255]
  major_text: [255, 255, 255]
  selectsymbol: ">>"

settings:
  discordrpc: true
  server: true
  caching: false
  volume: 95

servers:
  server.wylited:
    hostname: "wylihub"
    ip:
      -61
      -15
      -75
      -43
            "#;
            MuConfig::default()
        } else {
            MuConfig::default()
        }
    }
}
