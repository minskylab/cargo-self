Resource: Library Imports
  - std.path.PathBuf
  - async_openai.Client
  - cargo_self.engine.model.create_code_to_ro
  - cargo_self.engine.model.create_folder_to_ro
  - cargo_self.engine.planner.Action
  - cargo_self.engine.planner.Plan
  - tokio.main

Resource: Main Object
  - main

Operation: main
  - Get the root path from Cargo.toml file
  - Create a new plan with the root path
  - Create a new client
  - Iterate through each step in the plan
    - If the step is Action::CodeToRO:
      - Print "code to ro: <element>"
      - Create a request to convert code to ro using the element's content
      - If the request is successful:
        - Get the new_self_content from the response
        - Write the new_self_content to the element
    - If the step is Action::FolderToRO:
      - Print "folder to ro: <element>"
      - Print "neighbors: <children>"
      - Create a request to convert folder to ro using the element and children
      - If the request is successful:
        - Get the new_self_content from the response
        - Write the new_self_content to the element