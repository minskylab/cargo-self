Resource: Library Imports
- std.path.PathBuf
- async_openai.Client
- cargo_self.engine.model.create_code_to_ro
- cargo_self.engine.model.create_folder_to_ro
- cargo_self.engine.planner.Action
- cargo_self.engine.planner.Plan

Resource: main Function
  Operation: CodeToRO
    - Print "code to ro: {element}"
    - Create a request to convert code to ro
    - If the request is successful
        - Get the new self content from the response
        - Write the new self content to the element
  Operation: FolderToRO
    - Print "folder to ro: {element}"
    - Print "neighbors: {children}"
    - Create a request to convert folder to ro
    - If the request is successful
        - Get the new self content from the response
        - Write the new self content to the element