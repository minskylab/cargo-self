Resource: Library Imports
  - std.path.PathBuf
  - async_openai.Client
  - cargo_self.engine.constitution.ConstitutionDynamic
  - cargo_self.engine.planner.{Action, Plan}
  - dotenv.dotenv

Resource: Main Object
  - main function

Operation: main
  - Load environment variables from `.env` file
  - Define `root` as a `PathBuf` pointing to `./Cargo.toml`
  - Define `constitution_name` as a string "constitution.md"
  - Create a new `Plan` with `root`
  - Create a new `ConstitutionDynamic` with `constitution_name`
  - Create a new `Client`
  - Clone the nodes from the plan
  - Iterate over each step in the plan
    - If the step is `Action::CodeToRO`, do the following:
      - Print "code to ro: {element:?}"
      - Calculate the constitution rule for the element using `constitution_rule.calculate_for_element(element, &nodes)`
      - Create a chat request with the calculated constitution rule
      - Get the response from creating the chat request
      - Get the new self content from the first choice in the response
      - Write the new self content to the element
    - If the step is `Action::FolderToRO`, do the following:
      - Print "folder to ro: {element:?}"
      - Calculate the constitution rule for the element using `constitution_rule.calculate_for_element(element, &nodes)`
      - Create a chat request with the calculated constitution rule
      - Get the response from creating the chat request
      - Get the new self content from the first choice in the response
      - Write the new self content to the element