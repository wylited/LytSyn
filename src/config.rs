use directories::ProjectDirs;
use std::net::Ipv4Addr;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
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
            bar = [191, 239, 255],
            border = [191, 239, 255],
            minor_text = [191, 239, 255],
            major_text = [191, 239, 255],
            selectsymbol = [191, 239, 255],
            foldersymbol = [191, 239, 255],
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    discord: Boolean,
    network: Boolean,
    caching: Boolean,
    volume: i16,
}

impl Settings {
    pub fn new(ndiscord: Boolean, nnetwork: Boolean, ncaching: Boolean, nvolume: i16) -> Self {
        Self {
            discord = ndiscord,
            network = nnetwork,
            caching = ncaching,
            volume = nvolume,
        }
    }
    pub fn default() -> Self {
        Self {
            discord = true, 
            network = false,
            caching = true,
            volume = 100,
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
            owner = nowner,
            name = nname,
            ip = nip,
            port = nport,
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
    configdir: ProjectDirs,
}

impl RustMUInst {
    pub fn new(ver: String, unique: String, server: vec<Server>, file: Vec<CachedFile>, settings: Settings, themes: Theme, configpath: ProjectDirs) -> Self {
        Self {
            version = ver,
            UUID = unique,
            servers = server,
            files = file,
            settings = settings,
            theme = themes,
            configdir = configpath,
        }
    }

    pub fn get() -> RustMUInst {
        let config_path: &Path = Projectdirs::from("io", "wylited", "RustMU").config_dir();
        let config_file: &Path = config_path.join("config.toml");
        if !(config_file.exists()){
            let config = RustMUInst {
                version: "1.0.0".to_string(),
                UUID: "eba50a90-72e4-44d2-b8db-db1bafcc5d15".to_string(),
                servers: Vec::new(),
                files: Vec::new(),
                settings: Settings::default(),
                theme: Theme::default(),
                configdir = ProjectDirs::from("io", "wylited", "RustMU").config_dir(),
            };

            std::fs::write(config_path.join("config.toml"), config).expect("could not write to file");
            return config;
        } else {
            let config: RustMUInst = toml::from_file(config_file).unwrap();
            return config;
        }
    }
}