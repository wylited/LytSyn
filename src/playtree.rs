use glob::glob;
use std::ffi::OsString;
use std::path::{ PathBuf};
use tui::widgets::{List, ListItem};
use directories::UserDirs;

#[derive(Clone, Debug)]
pub enum FileType {
    F,
}

#[derive(Clone, Debug)]
pub struct RustMUFile {
    extension: OsString,
    filetype: FileType,
    name: OsString,
    path: PathBuf,
    parent: Option<PathBuf>,
}

impl RustMUFile {
    pub fn new(
        nextension: OsString,
        ftype: FileType,
        nname: OsString,
        npath: PathBuf,
        nparent: Option<PathBuf>,
    ) -> Self {
        Self {
            extension: nextension,
            filetype: ftype,
            name: nname,
            path: npath,
            parent: nparent,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RustMUTree {
    file_map: Vec<RustMUFile>,
}

impl RustMUTree {
    pub fn new(file_map: &[RustMUFile]) -> Self {
        Self {
            file_map: file_map.to_vec(),
        }
    }

    pub fn display<'a>(tree: RustMUTree) -> List<'a> {
        let files: Vec<RustMUFile> = tree.file_map;
        let mut items: Vec<ListItem<'a>> = Vec::new();

        for file in files {
            items.push(ListItem::new(String::from("│")+ &file.name.into_string().unwrap()));
        }

        List::new(items)
    }

    pub fn parse(other: bool, ofile: Option<&str>) -> RustMUTree {
        let mut tree = Vec::new();
        let user_dir = UserDirs::new().unwrap();
        let file: &str = &user_dir.home_dir().file_name().unwrap().to_str().unwrap();
        let mut iter = glob(&format!("C:/Users/{}/Music/**/*", file)).expect("Failed to read glob pattern");
        
        if other  == true {
            iter = glob(ofile.unwrap()).expect("Failed")
        }

        for entry in iter {
            match entry {
                Ok(path) => tree.push(RustMUFile::new(path.extension().unwrap_or_default().to_os_string(), FileType::F, path.file_stem().unwrap().to_os_string(), path.to_path_buf(), None)),
                Err(e) => println!("{:?}", e),
            }
        }

        RustMUTree::new(&tree)
    }
}