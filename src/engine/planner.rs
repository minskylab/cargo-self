use async_openai::{
    config::OpenAIConfig,
    types::{CreateChatCompletionRequest, CreateChatCompletionResponse},
    Client,
};
use cargo::{core::Shell, util::homedir};
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use std::{
    collections::{HashMap, HashSet},
    env::{self, current_dir},
    fmt::Debug,
    fs::{canonicalize, metadata, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
};

use super::constitution::ConstitutionDynamic;

pub trait SelfStatePersistence {
    fn save(&self, result: &AnalyzeResult);
    fn load(&self) -> Option<AnalyzeResult>;
}

pub struct Plan<Persistence>
where
    Persistence: SelfStatePersistence,
{
    // root: PathBuf,
    nodes: Vec<Element>,
    nodes_buffer: Vec<Element>,
    // constitution_name: String,

    // TODO: executor
    persistence: Persistence,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    CodeToRO {
        element: Element,
    },
    // ROToCode { element: Element },
    FolderToRO {
        element: Element,
        // children: Vec<Element>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Element {
    path: PathBuf,
    parent: PathBuf,
    modified: u64,
    is_file: bool,
    depth: usize,
    content: Option<String>,

    self_path: Option<PathBuf>,
    self_content: Option<String>,
    self_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionResult {
    pub llm_executed: bool,
    pub llm_input: Option<CreateChatCompletionRequest>,
    pub llm_result: Option<CreateChatCompletionResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputationUnit {
    pub element: Element,
    pub result: Option<ActionResult>,
}

impl ComputationUnit {
    pub fn new(
        element: Element,
        req: CreateChatCompletionRequest,
        res: CreateChatCompletionResponse,
        llm_executed: bool,
    ) -> Self {
        Self {
            element,
            result: Some(ActionResult {
                llm_executed,
                llm_input: Some(req),
                llm_result: Some(res),
            }),
        }
    }

    pub fn new_without_llm(element: Element) -> Self {
        Self {
            element,
            result: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeResult {
    pub computation_units: Vec<ComputationUnit>,
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

    pub fn find_children(&self, elements: &Vec<Element>) -> Vec<Element> {
        let mut children: Vec<Element> = Vec::new();

        for element in elements {
            if element.parent == self.path {
                children.push(element.clone());
            }
        }

        children
    }

    pub fn find_parent(&self, elements: &Vec<Element>) -> Option<Element> {
        for element in elements {
            if element.path == self.parent {
                return Some(element.clone());
            }
        }

        None
    }

    pub fn is_file(&self) -> bool {
        self.is_file
    }

    pub fn content(&self) -> String {
        self.content.clone().unwrap_or("".to_string())
    }

    pub fn self_content(&self) -> String {
        if let Some(path) = &self.self_path {
            std::fs::read_to_string(path).unwrap_or("".to_string())
        } else {
            "".to_string()
        }
    }
}

impl<Persistence> Iterator for &mut Plan<Persistence>
where
    Persistence: SelfStatePersistence,
{
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes_buffer.pop().map(|element| {
            if element.is_file {
                Action::CodeToRO { element }
            } else {
                Action::FolderToRO {
                    element: element.clone(),
                    // children: element.find_children(&self.nodes),
                }
            }
        })
    }
}

impl<Persistence> Plan<Persistence>
where
    Persistence: SelfStatePersistence,
{
    pub fn new(root: PathBuf, persistence: Persistence) -> Self {
        let elements = Plan::<Persistence>::explore(root.clone(), true);

        Self {
            nodes: elements.clone(),
            nodes_buffer: elements,
            persistence,
        }
    }

    pub fn nodes(&self) -> &Vec<Element> {
        &self.nodes
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
            .add_custom_ignore_filename(".selfignore")
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

        let (mut files, mut dirs): (Vec<&mut Element>, Vec<&mut Element>) =
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

            let mut self_content = None;

            if !new_self_file.exists() {
                std::fs::copy(element_path.clone(), new_self_file.clone()).unwrap();
            } else {
                self_content = Some(std::fs::read_to_string(new_self_file.clone()).unwrap());
            }

            let content = std::fs::read_to_string(element_path).unwrap_or("".to_string());

            element.content = Some(content.clone());

            element.self_path = Some(new_self_file);
            element.self_content = self_content;
            element.self_hash = Some(format!("{:x}", hash));

            parents_hm.insert(
                element.parent.clone(),
                parents_hm.get(&element.parent).unwrap() + 1,
            );
        });

        dirs.iter_mut().for_each(|element| {
            let element_path = element.path.clone();

            let self_root = Path::new("./.self");


            let (new_self_folder_path, self_content) = if element.depth != 0 {
                let p = element.parent.clone();

                let Ok(p) = p.strip_prefix(root) else {
                    return;
                };

                let new_self_path = self_root.join(p);

                // println!("new_self_path: {:?}", new_self_path);

                let mut new_folder_name = element_path.strip_prefix(root).unwrap().to_path_buf();

                // println!("new_folder_name: {:?}", new_folder_name);

                add_extension(&mut new_folder_name, "md");

                let new_self_folder = self_root.join(new_folder_name);

                // println!("new_self_folder: {:?}", new_self_folder);

                std::fs::create_dir_all(new_self_path).unwrap();

                let mut self_content = None;


                if !new_self_folder.exists() {
                    let placeholder_content = "This is a placeholder for a folder. It will be replaced by the result of the following command:\n\n```llm\nfolder to ro\n```";
                    std::fs::write(new_self_folder.clone(), placeholder_content).unwrap();
                } else {
                    self_content = Some(std::fs::read_to_string(new_self_folder.clone()).unwrap());
                }

                (new_self_folder, self_content)
            } else {
                let new_folder_name = self_root.to_path_buf().join(".self.md");

                let mut self_content = None;

                if !new_folder_name.exists() {
                    let placeholder_content = "This is a placeholder for a folder. It will be replaced by the result of the following command:\n\n```llm\nfolder to ro\n```";
                    std::fs::write(new_folder_name.clone(), placeholder_content).unwrap();
                } else {
                    self_content = Some(std::fs::read_to_string(new_folder_name.clone()).unwrap());
                }

                (new_folder_name, self_content)
            };

            element.self_path = Some(new_self_folder_path);
            element.self_content = self_content;
            // element.self_hash = Some(format!("{:x}", hash));
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

        let mut freezed_elements_list = final_elements_list.clone();
        let mut final_elements_list_clone = final_elements_list.clone();

        final_elements_list.reverse();

        for (i, element) in freezed_elements_list.iter_mut().enumerate() {
            if element.is_file {
                // println!("file: {element:?}");

                continue;
            }

            // println!("directory: {element:?}");

            let dir_children = element.find_children(&final_elements_list_clone);

            // println!("{dir_children:?}");

            let a = dir_children
                .iter()
                .map(|e| e.self_hash.clone().unwrap())
                .collect::<Vec<String>>()
                .join("-");

            let hash = format!("{:x}", Sha256::digest(a.as_bytes()));

            final_elements_list_clone[i].self_hash = Some(hash.clone());
            element.self_hash = Some(hash);
        }

        freezed_elements_list.reverse();

        freezed_elements_list
        // final_elements_list
    }

    pub async fn walk_elements(
        &mut self,
        dynamic: &ConstitutionDynamic,
        client: &Client<OpenAIConfig>,
    ) -> AnalyzeResult {
        let nodes = self.nodes().clone();

        let last_state = self.persistence.load();

        let hashmap_last_state = last_state.map(|s| {
            let computation_units = s.computation_units;

            computation_units
                .into_iter()
                .map(|unit| (unit.element.path.clone(), unit))
                .collect::<HashMap<PathBuf, ComputationUnit>>()
        });

        let mut results = Vec::new();

        for step in self.into_iter() {
            match step {
                Action::CodeToRO { element } => {
                    print!("code to ro: {element:?} ");

                    if let Some(hashmap) = hashmap_last_state.clone() {
                        let last_hash = hashmap
                            .get(&element.path)
                            .map(|unit| unit.element.self_hash.clone().unwrap_or("".to_string()))
                            .unwrap_or("".to_string());

                        if last_hash == element.self_hash.clone().unwrap() {
                            println!("[SKIPPED]");
                            results.push(ComputationUnit::new_without_llm(element));
                            continue;
                        }
                    }

                    let request = dynamic.calculate(&element, &nodes);

                    let response = client.chat().create(request.clone()).await.unwrap();

                    let new_self_content = response
                        .choices
                        .first()
                        .unwrap()
                        .message
                        .content
                        .to_owned()
                        .unwrap();

                    element.write_new_self_content(new_self_content);

                    println!("[COMPUTED]");
                    results.push(ComputationUnit::new(element, request, response, true));
                }
                Action::FolderToRO { element } => {
                    print!("folder to ro: {element:?} ");

                    if element.depth == 0 {
                        let children = element.find_children(&nodes);
                        println!("neighbors: {:?}", children);
                    }

                    if let Some(hashmap_last_state) = hashmap_last_state.clone() {
                        let last_hash = hashmap_last_state
                            .get(&element.path)
                            .map(|unit| unit.element.self_hash.clone().unwrap_or("".to_string()))
                            .unwrap_or("".to_string());

                        if last_hash == element.self_hash.clone().unwrap() {
                            println!("[SKIPPED]");
                            results.push(ComputationUnit::new_without_llm(element));
                            continue;
                        }
                    }

                    let request = dynamic.calculate(&element, &nodes);

                    let response = client.chat().create(request.clone()).await.unwrap();

                    let new_self_content = response
                        .choices
                        .first()
                        .unwrap()
                        .message
                        .content
                        .to_owned()
                        .unwrap();

                    element.write_new_self_content(new_self_content);

                    // results.push(ComputationUnit::new_without_llm(element));
                    println!("[COMPUTED]");
                    results.push(ComputationUnit::new(element, request, response, true));
                }
            }
        }

        let result = AnalyzeResult {
            computation_units: results,
        };

        self.persistence.save(&result);

        result
    }
}

impl AnalyzeResult {
    pub fn new() -> Self {
        Self {
            computation_units: Vec::new(),
        }
    }

    pub fn consolidate(&self) -> String {
        let mut consolidated = String::new();

        for unit in self.computation_units.clone() {
            let element = unit.element;

            let self_content = element.self_content() + "\n";

            consolidated.push_str(&self_content);
        }

        consolidated
    }
}

impl Default for AnalyzeResult {
    fn default() -> Self {
        Self::new()
    }
}

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
