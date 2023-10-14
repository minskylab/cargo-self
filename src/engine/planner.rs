use cargo::{core::Shell, util::homedir};
use ignore::WalkBuilder;
use sha2::{Digest, Sha256};

use std::{
    collections::{HashMap, HashSet},
    env,
    fmt::Debug,
    fs::{canonicalize, metadata, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
};

pub struct Plan {
    // root: PathBuf,
    nodes: Vec<Element>,
    elements_list: Vec<Element>,
}

#[derive(Debug)]
pub enum Action {
    CodeToRO {
        element: Element,
    },
    // ROToCode { element: Element },
    FolderToRO {
        element: Element,
        neighbors: Vec<Element>,
    },
}

impl Iterator for Plan {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements_list.pop().map(|element| {
            if element.is_file {
                Action::CodeToRO { element }
            } else {
                Action::FolderToRO {
                    element: element.clone(),
                    neighbors: element.find_children(&self.nodes),
                }
            }
        })
    }
}

#[derive(Clone)]
pub struct Element {
    path: PathBuf,
    parent: PathBuf,
    modified: u64,
    is_file: bool,
    depth: usize,
    pub content: Option<String>,

    self_path: Option<PathBuf>,
    self_content: Option<String>,
    self_hash: Option<String>,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("path", &self.path)
            // .field("parent", &self.parent)
            // .field("modified", &self.modified)
            // // .field("is_file", &self.is_file)
            .field("depth", &self.depth)
            // .field("self_path", &self.self_path)
            // .field("self_hash", &self.self_hash)
            .finish()
    }
}

impl Element {
    pub fn write_new_self_content(&self, new_self_content: String) -> bool {
        if !self.is_file {
            return false;
        }

        let self_path = self.self_path.as_ref().unwrap();

        std::fs::write(self_path, new_self_content).unwrap();

        true
    }

    pub fn is_dirty(&self) -> bool {
        if !self.is_file {
            return false;
        }

        let element_path = self.path.as_path();

        let mut file = File::open(element_path).unwrap();
        let mut sha256 = Sha256::new();

        io::copy(&mut file, &mut sha256).unwrap();

        let hash = sha256.finalize();

        let self_hash = self.self_hash.clone().unwrap();
        println!("old hash: {}", self_hash);
        println!("new hash: {:x}", hash);

        self_hash != format!("{:x}", hash)
    }

    pub fn find_children(&self, elements: &Vec<Element>) -> Vec<Element> {
        let mut children: Vec<Element> = Vec::new();

        // println!("elements: {:?}", elements);

        for element in elements {
            if element.parent == self.path {
                children.push(element.clone());
            }
        }

        children
    }
}

impl Plan {
    pub fn new(root: PathBuf) -> Self {
        let elements = Plan::explore(root.clone(), true);

        Self {
            nodes: elements.clone(),
            elements_list: elements,
        }
    }

    fn explore(root: PathBuf, with_default_shell: bool) -> Vec<Element> {
        let buf = BufWriter::new(Vec::new());

        let config = if with_default_shell {
            cargo::util::config::Config::default().unwrap()
        } else {
            let shell = Shell::from_write(Box::new(buf));
            let cwd = env::current_dir().unwrap();
            let homedir = homedir(&cwd).unwrap();

            cargo::util::config::Config::new(shell, cwd, homedir)
        };

        let manifest_path = canonicalize(root).unwrap();

        let ws = cargo::core::Workspace::new(&manifest_path, &config).unwrap();

        let root = ws.root();
        // println!("root: {:?}", root);

        let mut elements: Vec<Element> = WalkBuilder::new(root)
            .hidden(true)
            .build()
            .map(|entry| {
                let entry = entry.unwrap();

                let file_metadata = metadata(entry.path()).unwrap();
                let modified = file_metadata.modified().unwrap();
                let file_type = file_metadata.file_type();

                Element {
                    path: entry.path().to_path_buf(),
                    parent: entry.path().parent().unwrap().to_path_buf(),
                    modified: modified.elapsed().unwrap().as_secs(),
                    is_file: file_type.is_file(),
                    depth: entry.depth(),
                    content: None,

                    self_path: None,
                    self_content: None,
                    self_hash: None,
                }
            })
            .collect();

        elements.sort_by(|a, b| {
            a.modified
                .partial_cmp(&b.modified)
                .map(|ordering| ordering.reverse())
                .map(|ordering| ordering.max(a.depth.cmp(&b.depth)).reverse())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let (mut files, dirs): (Vec<&mut Element>, Vec<&mut Element>) =
            elements.iter_mut().partition(|element| element.is_file);

        let parents: HashSet<_> = dirs.iter().map(|d| d.path.clone()).collect();

        let mut parents_hm: HashMap<_, _> = parents.iter().map(|p| (p.clone(), 0)).collect();

        files.iter_mut().for_each(|element| {
            let element_path = element.path.clone();

            let mut file = File::open(element_path.clone()).unwrap();
            let mut sha256 = Sha256::new();

            io::copy(&mut file, &mut sha256).unwrap();

            let hash = sha256.finalize();

            // println!("hash is: {:x}", hash);

            let self_root = Path::new("./.self");

            let p = element.parent.clone();

            let new_self_path = self_root.join(p.strip_prefix(root).unwrap());

            // println!("new dir: {:?}", new_self_path);

            let mut new_file_name = element_path.strip_prefix(root).unwrap().to_path_buf();

            add_extension(&mut new_file_name, "md");

            let new_self_file = self_root.join(new_file_name);

            std::fs::create_dir_all(new_self_path).unwrap();

            if !new_self_file.exists() {
                std::fs::copy(element_path.clone(), new_self_file.clone()).unwrap();
            }

            let content = std::fs::read_to_string(element_path).unwrap_or("".to_string());

            element.content = Some(content.clone());

            element.self_path = Some(new_self_file);
            element.self_content = Some(content);
            element.self_hash = Some(format!("{:x}", hash));

            parents_hm.insert(
                element.parent.clone(),
                parents_hm.get(&element.parent).unwrap() + 1,
            );
        });

        let mut final_elements_list: Vec<Element> = Vec::new();

        files.iter().for_each(|x| {
            parents_hm.insert(x.parent.clone(), parents_hm.get(&x.parent).unwrap() - 1);

            final_elements_list.push((**x).clone());

            if parents_hm.get(&x.parent).unwrap() == &0 {
                let parent = dirs.iter().find(|y| y.path == x.parent).unwrap().to_owned();

                final_elements_list.push((**parent).clone());
            }
        });

        final_elements_list.reverse();

        final_elements_list
    }

    pub fn analyze(&self) -> AnalyzeResult {
        AnalyzeResult {}
    }
}

pub struct AnalyzeResult {}

fn add_extension(path: &mut std::path::PathBuf, extension: impl AsRef<std::path::Path>) {
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            ext.push(".");
            ext.push(extension.as_ref());
            path.set_extension(ext)
        }
        None => path.set_extension(extension.as_ref()),
    };
}
