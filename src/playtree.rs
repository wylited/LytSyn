use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

pub enum FileType {
    F,
    D,
}

pub struct RustMUFile {
    extension: String,
    filetype: FileType,
    name: String,
    path: PathBuf,
    parent: Some(File),
}

impl RustMUFile {
    pub fn new(
        nextension: String,
        ftype: FileType,
        nname: String,
        npath: PathBuf,
        nparent: Some(File),
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
pub struct RustMUTree {
    dir_map: HashMap<i32, RustMUFile>
}
impl RustMUTree {
    pub fn new(file_map: HashMap<i32, RustMUFile>) -> Self {
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

    pub fn parse(musicdir: &Path) -> RustMUTree {
        let paths = fs::read_dir(musicdir).unwrap();
        let mut tree: Vec<RustMUFile> = Vec::new();

        let filecheck = |dir: &Path, parent: Some(RustMUFile)| -> RustMUFile {
            if !(path.is_file()){
                tree.push(filecheck(path, parent));
                return RustMUFile::new(path.extension(), FileType::D, path.file_name().to_string, path, parent);
            } else {
                return RustMUFile::new(path.extension(), FileType::F, path.file_name().to_string, path, parent);
            }
        };
        for path in paths {
            tree.push(filecheck(path, None));
            // O((N logN)^2)
        } 

        return tree;
    }
}

