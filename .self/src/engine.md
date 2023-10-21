Resource: Library Imports
  - std, async_openai, serde, sha2, ignore, serde_json, cargo, async_graphql, chrono, uuid, handlebars, std::fs, std::path, serde_yaml

Resource: ConstitutionDynamic Struct
  - name: String

Resource: ElementMinimized Struct
  - is_file: bool
  - path: String
  - content: String
  - children: Vec<ElementMinimized>

Resource: ConstitutionPayload Struct
  - element: ElementMinimized

Resource: JsonMemoryPersistence Struct
  - path: PathBuf

Resource: Action Enum
  - CodeToRO
    - element: Element
  - FolderToRO
    - element: Element

Resource: Element Struct
  - path: PathBuf
  - parent: PathBuf
  - modified: u64
  - is_file: bool
  - depth: usize
  - content: Option&lt;String&gt;
  - self_path: Option&lt;PathBuf&gt;
  - self_content: Option&lt;String&gt;
  - self_hash: Option&lt;String&gt;

Resource: ActionResult Struct
  - llm_executed: bool
  - llm_input: Option&lt;CreateChatCompletionRequest&gt;
  - llm_result: Option&lt;CreateChatCompletionResponse&gt;

Resource: ComputationUnit Struct
  - element: Element
  - result: Option&lt;ActionResult&gt;

Resource: AnalyzeResult Struct
  - computation_units: Vec&lt;ComputationUnit&gt;

Operation: ConstitutionDynamic.new
  - Create a new ConstitutionDynamic object with a given name

Operation: ConstitutionDynamic.calculate
  - Generate a CreateChatCompletionRequest for a specific element and project nodes

Operation: Element.write_new_self_content
  - Write a new content to the element's self file

Operation: Element.relative_path
  - Get the relative path of the element

Operation: Element.find_children
  - Find the children elements of the current element

Operation: Element.find_parent
  - Find the parent element of the current element

Operation: Element.is_file
  - Check if the element represents a file

Operation: Element.content
  - Get the content of the element

Operation: Element.self_content
  - Get the content of the element's self file

Operation: Plan.new
  - Create a new Plan object with a root path and persistence

Operation: Plan.nodes
  - Get the list of nodes