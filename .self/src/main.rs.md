- ```yaml
  Resource: Library Imports
      - std::path::PathBuf, async_openai::types::CreateChatCompletionRequestArgs, async_openai::Client, cargo_self::engine::model::create_new_default_request, cargo_self::engine::planner::{Action, Plan}, tokio::main

  Resource: Main Function (main)
      Operation: plan
          - Create a new plan object for the specified Cargo project root directory
      Operation: client
          - Create a new OpenAI client
      Operation: for loop
          - Iterate over each step in the plan
      Operation: match step
          - Match the type of step and execute the associated action
      Operation: Action::CodeToRO
          - If the step is a CodeToRO action, perform the following operations:
              - Print "code to ro" followed by the element
              - Create a new default completion request with the element's content
              - Create a chat completion using the client with the request
              - Retrieve the new self content from the response
              - Write the new self content to the element
              - Print "foo" followed by the foo value
      Operation: Action::FolderToRO
          - If the step is a FolderToRO action, perform the following operations:
              - Print "folder to ro" followed by the element

  Commented Out Code:
      - cargo::ops::CompileOptions::new(..)
      - Set example to named workspace member
      - Print options.spec
      - Create a new OpenAI chat completion request with specific parameters
      - Create a chat completion using the client with the request
      - Get the package name from the workspace
      - Print each target's source path and target itself
      - Iterate over dependencies and print each one
      - Compile the workspace with the options
      - Print the resulting host value
  ```