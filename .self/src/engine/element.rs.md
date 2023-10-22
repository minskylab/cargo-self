Resource: Library Imports
  - Dependencies: serde, std

Resource: Element Struct
  - Fields:
    - `path`: PathBuf
    - `parent`: PathBuf
    - `modified`: u64
    - `is_file`: bool
    - `depth`: usize
    - `content`: Option<String>
    - `self_path`: Option<PathBuf>
    - `self_content`: Option<String>
    - `self_hash`: Option<String>

Operation: write_new_self_content
  - Description: Write new content to the `self_path` file of the Element

Operation: relative_path
  - Description: Get the relative path of the Element

Operation: find_children
  - Description: Find the children Elements of the given Elements array based on their parent path matching the Element's `path`

Operation: find_parent
  - Description: Find the parent Element of the given Elements array based on their path matching the Element's `parent`

Operation: self_content
  - Description: Read and return the content of the file at the `self_path` of the Element if it exists, otherwise return an empty string