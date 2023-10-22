Resource: Library Imports
  - async_openai, cargo, serde, serde_json, serde_yaml, std, candid, serde_path_to_error, clap, async_trait, futures, percent_encoding, tokio, base64, snafu, snafu_derive, tokio_util, hyper, hyper_tls, hyperx, lazy_static, tracing, tracing_futures, prettytable, chrono, tokio_test, assert_matches, mockito, k8s_openapi

Resource: Input Filters
  - None

Resource: Main Object
  - ConstitutionDynamic

Operation: new
  - Create a new instance of ConstitutionDynamic with the given name

Operation: constitution_filepath
  - Calculate where the constitution file should be located for a given Element

Operation: system_input_data
  - Extract the system and input prompts data from a given Element and a list of nodes

Operation: calculate
  - Calculate the CreateChatCompletionRequest for a given Element and a list of nodes

Resource: Element
  - Structure: path, parent, modified, is_file, depth, content, self_path, self_content, self_hash

Operation: write_new_self_content
  - Write new self content to the Element's self path

Operation: relative_path
  - Get the relative path of the Element

Operation: find_children
  - Find the children Elements of the current Element

Operation: find_parent
  - Find the parent Element of the current Element

Operation: self_content
  - Get the self content of the Element

Resource: Plan
  - Structure: nodes, nodes_buffer, persistence

Operation: new
  - Create a new instance of Plan with the given root path and persistence

Operation: nodes
  - Get the list of nodes

Operation: explore
  - Explore the file system structure starting from the given root path

Operation: process
  - Process the Plan by calculating the CreateChatCompletionRequest and CreateChatCompletionResponse for each Action

Resource: JsonMemoryPersistence
  - Structure: path

Operation: new
  - Create a new instance of JsonMemoryPersistence with the given path

Resource: ActionResult
  - Structure: llm_executed, llm_input, llm_result

Resource: ComputationUnit
  - Structure: element, result

Operation: new
  - Create a new instance of ComputationUnit with the provided Element, CreateChatCompletionRequest, CreateChatCompletionResponse, and llm_executed flag

Operation: new_without_llm