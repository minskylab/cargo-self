use cargo::{core::Shell, util::homedir};
use ignore::WalkBuilder;
use std::{
    collections::BinaryHeap,
    env,
    fs::{canonicalize, metadata},
    io::BufWriter,
    path::{Path, PathBuf},
};

pub struct Plan {
    root: &'static Path,
}

pub enum Action {
    CodeToRO {
        parent: &'static Path,
        code: &'static Path,
    },
    ROToCode {
        ro: &'static Path,
    },
    FolderToRO(&'static Path),
}

impl Iterator for Plan {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug)]
struct Element {
    path: PathBuf,
    parent: PathBuf,
    modified: u64,
    is_file: bool,
}

impl std::cmp::Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.modified.cmp(&other.modified)
    }
}

impl std::cmp::PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for Element {}

impl std::cmp::PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.modified == other.modified
    }
}

impl Plan {
    pub fn new(root: &'static Path) -> Self {
        let s = Self { root };

        s.explore(true);

        s
    }

    fn explore(&self, with_default_shell: bool) {
        let buf = BufWriter::new(Vec::new());

        let config = if with_default_shell {
            cargo::util::config::Config::default().unwrap()
        } else {
            let shell = Shell::from_write(Box::new(buf));
            let cwd = env::current_dir().unwrap();
            let homedir = homedir(&cwd).unwrap();

            cargo::util::config::Config::new(shell, cwd, homedir)
        };

        let manifest_path = canonicalize(PathBuf::from("./Cargo.toml")).unwrap();

        let ws = cargo::core::Workspace::new(&manifest_path, &config).unwrap();

        let root = ws.root();
        println!("root: {:?}", root);

        let a: BinaryHeap<Element> = WalkBuilder::new(root)
            .hidden(true)
            .build()
            .map(|entry| {
                let entry = entry.unwrap();

                let file_metadata = metadata(entry.path()).unwrap();
                let modified = file_metadata.modified().unwrap();
                let file_type = file_metadata.file_type();

                // let file_parent = entry.path().parent().unwrap();

                // println!(
                //     "entry: {:?} | is file: {} | last modified: {} | parent: {:?}",
                //     entry.path(),
                //     file_type.is_file(),
                //     modified.elapsed().unwrap().as_secs(),
                //     file_parent,
                // );

                Element {
                    path: entry.path().to_path_buf(),
                    parent: entry.path().parent().unwrap().to_path_buf(),
                    modified: modified.elapsed().unwrap().as_secs(),
                    is_file: file_type.is_file(),
                }
            })
            .collect();

        let mut elements = a.into_sorted_vec();

        elements.iter().for_each(|element| {
            println!("element: {:?}", element);
        });

        elements.sort_by(|a, b| a.is_file.cmp(&b.is_file));
        elements.reverse();

        elements.iter().for_each(|element| {
            println!("element: {:?}", element);
        });
    }
}
