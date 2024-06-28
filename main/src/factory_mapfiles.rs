use std::{collections::HashMap, io::Read, ops::{Deref, DerefMut}};

use walkdir::WalkDir;


#[derive(Debug, Clone)]
pub struct MapFile {
    pub content: String,
    pub fullpath: String,
}

impl MapFile {
    pub fn new(content: &str, fullpath: &str) -> Self {
        MapFile {
            content: content.to_string(),
            fullpath: fullpath.to_string(),
        }
    }
}

pub fn init_mapfiles() -> HashMap<String, MapFile>  {
    let mut factory = HashMap::new();
    for entry in WalkDir::new("D:\\mwnd\\RustMud\\maps")
        .into_iter().filter_map(|e| e.ok())  {
        let filename = entry.file_name().to_str().unwrap();
        let fullpath = entry.path().display().to_string();       

        if fullpath.contains(".txt") {
            let mut read = utils::load_file(&fullpath);
            let mut content = String::new();
            read.read_to_string(&mut content);

            let mut mapfile = MapFile::new(&content, &fullpath);

            factory.insert(filename.to_string(), mapfile);

        }
    }

    factory
}
