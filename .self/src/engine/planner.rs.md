Resource: Library Imports
  - cargo, ignore, sha2, std

Resource: Plan Struct
  Operation: Iterator
    - Iterate over elements in the elements_list
  Operation: new
    - Initialize a new Plan struct with a root path
  Operation: analyze
    - Analyze the plan and return an AnalyzeResult struct

Resource: Action Enum
  - CodeToRO variant
    - Convert a code element to a Resource-Operation element
  - FolderToRO variant
    - Convert a folder element to a Resource-Operation element

Resource: Element Struct
  Operation: write_new_self_content
    - Write new self_content to the file represented by the element
  Operation: is_dirty
    - Check if the file represented by the element has been modified
  Operation: find_children
    - Find the child elements of the element from a list of elements