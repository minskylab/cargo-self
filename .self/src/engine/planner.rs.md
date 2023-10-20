Resource: Library Imports
  - async_openai
  - cargo
  - ignore
  - serde
  - sha2
  - std

Resource: Plan Struct
  Operation: new
    - Create a new instance of the Plan struct
  Operation: nodes
    - Get a reference to the nodes vector
  Operation: walk_elements
    - Perform a walk over the elements and calculate the results

Resource: ComputationUnit Struct
  Operation: new
    - Create a new instance of the ComputationUnit struct
  Operation: new_without_llm
    - Create a new instance of the ComputationUnit struct without language model execution

Resource: AnalyzeResult Struct

Resource: Element Struct
  Operation: write_new_self_content
    - Write new self content to the file
  Operation: relative_path
    - Get the relative path of the element
  Operation: find_children
    - Find children elements of the current element
  Operation: find_parent
    - Find the parent element of the current element
  Operation: is_file
    - Check if the element is a file
  Operation: content
    - Get the content of the element

Resource: Action Enum
  - CodeToRO
  - FolderToRO