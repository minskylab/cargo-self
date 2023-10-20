Resource: Library Imports
  - async_openai, ChatCompletionRequestMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs, Role, super, planner::Element, prompts::DEFAULT_SYSTEM_PROMPT, fs, serde, std, std::env, std::fmt, std::fs, std::io, std::path, std::collections, std::collections::{HashMap, HashSet}, async_openai::config::OpenAIConfig, async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionResponse}, async_openai::Client, cargo::{core::Shell, util::homedir}, cargo::util::config::Config, cargo::core::Workspace, ignore::WalkBuilder, serde::{Deserialize, Serialize}, sha2::{Digest, Sha256}, handlebars::Handlebars

Resource: Module constitution
  Resource: ConstitutionDynamic
    - name: String
  Resource: Element
    - path: PathBuf
    - parent: PathBuf
    - modified: u64
    - is_file: bool
    - depth: usize
    - content: Option<String>
    - self_path: Option<PathBuf>
    - self_content: Option<String>
    - self_hash: Option<String>

Resource: Module model
  Resource: create_folder_to_ro
    - _element: Element
    - children: Vec<Element>
  Resource: CreateChatCompletionRequest
    - request: CreateChatCompletionRequestArgs
    - sources: Vec<String>
  Resource: ConstitutionPayload
    - element: ElementMinimized
  Resource: ElementMinimized
    - is_file: bool
    - path: String
    - content: String
    - children: Vec<ElementMinimized>

Resource: Module prompts
  Resource: DEFAULT_SYSTEM_PROMPT
    - "Your task is..."
  Resource: DEFAULT_SYSTEM_PROMPT
    - "Your task is..."

Resource: Module version
  Resource: VERSION
    - "0.1.4"

Resource: Module persistence
  Resource: JsonMemoryPersistence
    - path: PathBuf
  Resource: JsonMemoryPersistence
    - path: PathBuf

Resource: Module planner
  Resource: Plan
    - nodes: Vec<Element>
    - nodes_buffer: Vec<Element>
    - persistence: Persistence
  Resource: Action
    - CodeToRO: {element: Element}
    - FolderToRO: {element: Element}
  Resource: ActionResult
    - llm_executed: bool
