use serde::{Deserialize, Serialize};

use std::{env::current_dir, fmt::Debug, path::PathBuf};

#[derive(Clone, Serialize, Deserialize)]
pub struct Element {
    pub path: PathBuf,
    pub parent: PathBuf,
    pub modified: u64,
    pub is_file: bool,
    pub depth: usize,
    pub content: Option<String>,

    pub self_path: Option<PathBuf>,
    pub self_content: Option<String>,
    pub self_hash: Option<String>,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            // .field("path", &self.path)
            // .field("parent", &self.parent)
            // .field("modified", &self.modified)
            // // .field("is_file", &self.is_file)
            .field("self", &self.self_path)
            // .field("self_hash", &self.self_hash)
            .field("depth", &self.depth)
            .finish()
    }
}

impl Element {
    pub fn write_new_self_content(&self, new_self_content: String) -> bool {
        let self_path = self.self_path.clone().unwrap();

        // if !self.is_file {
        //     // i.e. is a directory
        //     add_extension(&mut self_path, "md");
        // }

        std::fs::write(self_path, new_self_content).unwrap();
        true
    }

    pub fn relative_path(&self) -> PathBuf {
        let root = current_dir().unwrap();

        self.path.strip_prefix(root).unwrap().to_path_buf()
    }

    pub fn find_children(&self, elements: &[Element]) -> Vec<Element> {
        let mut children: Vec<Element> = Vec::new();

        for element in elements {
            if element.parent == self.path {
                children.push(element.clone());
            }
        }

        children
    }

    pub fn find_parent(&self, elements: &[Element]) -> Option<Element> {
        for element in elements {
            if element.path == self.parent {
                return Some(element.clone());
            }
        }

        None
    }

    pub fn self_content(&self) -> String {
        if let Some(path) = &self.self_path {
            std::fs::read_to_string(path).unwrap_or("".to_string())
        } else {
            "".to_string()
        }
    }
}
