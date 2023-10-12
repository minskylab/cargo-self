```yaml
Resource: Library Imports
  - cargo
  - ignore
  - sha2
  - std

Resource: Plan Struct
  Operation: Iterator
    - Get the next action to perform in the plan
  Operation: new
    - Create a new Plan with a specified root path
  
Resource: Action Enum
  Variant: CodeToRO
    - Convert code element to RO format
  Variant: FolderToRO
    - Convert folder element to RO format

Resource: Element Struct
  Operation: write_new_self_content
    - Write new self content to the element
  Operation: Debug
    - Format the Element struct for debugging purposes

Resource: AnalyzeResult Struct

Function: add_extension
  - Add an extension to a path
```