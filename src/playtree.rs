use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};

#[derive(Clone)]
pub enum FileType {
    F,
    D,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct RustMUTree {
    file_map: Vec<RustMUFile>,
}

impl RustMUTree {
    pub fn new(file_map: &Vec<RustMUFile>) -> Self {
        Self {
            file_map: file_map.to_vec(),
        }
    }

    pub fn display<'a>(tree: RustMUTree) -> List<'a> {
        let files: Vec<RustMUFile> = tree.file_map;
        let mut items: Vec<ListItem<'a>> = Vec::new();

        for file in files {
            items.push(ListItem::new(file.name.into_string().unwrap()));
        }

        List::new(items)
            .block(Block::default().title("Playlist").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
    }

    pub fn parse(musicdir: PathBuf) -> RustMUTree {
        let paths = fs::read_dir(musicdir).unwrap();
        let mut tree = &Vec::new();

        fn filecheck<'a>(path: &'a PathBuf, parent: Option<PathBuf>, original: &Vec<RustMUFile>) -> &'a Vec<RustMUFile> {
            let mut changed: Vec<RustMUFile> = original.to_vec();
            if !(path.is_file()){
                changed.extend(filecheck(path, Some(path.to_path_buf()), original).iter().cloned());
                changed.push(RustMUFile::new(path.extension().unwrap().to_os_string(), FileType::D, path.file_name().unwrap().to_os_string(), path.to_path_buf(), parent));
                let returnval = &changed.to_vec();
                return returnval;
            } else {
                changed.push(RustMUFile::new(path.extension().unwrap().to_os_string(), FileType::F, path.file_name().unwrap().to_os_string(), path.to_path_buf(), parent));
                let returnval = &changed.to_vec();
                return returnval;
            } 
        }

        for path in paths {
            let tree = filecheck(&path.unwrap().path(), None, &tree);
        }
        return RustMUTree::new(tree);
    }
}

