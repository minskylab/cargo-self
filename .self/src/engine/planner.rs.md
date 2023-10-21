Resource: Library Imports
  - async_openai
  - cargo
  - ignore
  - serde
  - sha2
  - std

Resource: SelfStatePersistence Trait
  Operation: save
    - Save the result of the analysis.
  Operation: load
    - Load the previous analysis result.

Resource: Plan Struct
  - persistence: Persistence
  - nodes: Vec<Element>
  - nodes_buffer: Vec<Element>

Resource: Action Enum
  - CodeToRO Variant
    - element: Element
  - FolderToRO Variant
    - element: Element

Resource: Element Struct
  - path: PathBuf
  - parent: PathBuf
  - modified: u64
  - is_file: bool
  - depth: usize
  - content: Option<String>
  - self_path: Option<PathBuf>
  - self_content: Option<String>
  - self_hash: Option<String>

Resource: ActionResult Struct
  - llm_executed: bool
  - llm_input: Option<CreateChatCompletionRequest>
  - llm_result: Option<CreateChatCompletionResponse>

Resource: ComputationUnit Struct
  - element: Element
  - result: Option<ActionResult>

Resource: AnalyzeResult Struct
  - computation_units: Vec<ComputationUnit>

Operation: Element.write_new_self_content
  - Write new self content to the file.

Operation: Element.relative_path
  - Get the relative path of the element.

Operation: Element.find_children
  - Find children of the element from the list of elements.

Operation: Element.find_parent
  - Find the parent of the element from the list of elements.

Operation: Element.is_file
  - Check if the element is a file.

Operation: Element.content
  - Get the content of the element.

Operation: Element.self_content
  - Get the self content of the element.

Operation: Plan::new
  - Create a new Plan instance.

Operation: Plan::nodes
  - Get the list of nodes.

Operation: Plan::explore
  - Explore the root directory and return a list of elements.

Operation: Plan::walk_elements
  - Walk through the elements and perform analysis using dynamic constitution and OpenAI client.

Operation: AnalyzeResult.consolidate
  - Consolidate the self content of the computation units into a single string.