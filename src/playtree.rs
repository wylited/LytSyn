use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum FileType {
    F,
    D,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct RustMUFile {
    extension: OsString,
    filetype: FileType,
    name: OsString,
    path: PathBuf,
    parent: Option<File>,
}

impl RustMUFile {
    pub fn new(
        nextension: OsString,
        ftype: FileType,
        nname: OsString,
        npath: PathBuf,
        nparent: Option<File>,
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

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct RustMUTree {
    dir_map: Vec<RustMUFile>
}
impl RustMUTree {
    pub fn new(file_map: Vec<RustMUFile>) -> Self {
        Self {
            dir_map: file_map,
        }
    }

    pub fn display(items: Vec<ListItem>) -> List {
        List::new(items)
            .block(Block::default().title("Playlist").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
    }

    pub fn parse(musicdir: PathBuf) -> RustMUTree {
        let paths = fs::read_dir(musicdir).unwrap();
        let mut tree= Vec::new();

        fn filecheck(path: PathBuf, parent: Option<RustMUFile>, original: Vec<RustMUFile>) -> Vec<RustMUFile> {
            if !(path.is_file()){
                original = filecheck(path, parent, original);
                original.push(RustMUFile::new(path.extension().unwrap().to_os_string(), FileType::D, path.file_name().unwrap().to_os_string(), path, parent));
                return original;
            } else {
                original.push(RustMUFile::new(path.extension().unwrap().to_os_string(), FileType::F, path.file_name().unwrap().to_os_string(), path, parent));
                return original;
            } 
        }

        for path in paths {
            filecheck(path.unwrap().path(), None, tree);
            // O((N logN)^2)
        } 

        return RustMUTree::new(tree);
    }
}

