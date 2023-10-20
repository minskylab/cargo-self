Resource: Library Imports
  - std, async_openai, cargo_self, tokio

Resource: Main Object
  - struct ElementRule

Resource: Operation in Main Object
  - None

Resource: Plan Object
  - Operation: new
    - Create a new Plan object
  
Resource: ConstitutionDynamic Object
  - Operation: new
    - Create a new ConstitutionDynamic object with constitution_name as a parameter
  
Resource: Client Object
  - Operation: new
    - Create a new Client object
  
Resource: PathBuf Object
  - Operation: from
    - Create a new PathBuf object with "./Cargo.toml" as a parameter
  
Resource: async_main()
  - Operation: async_main
    - The main async function that serves as the entry point of the program
  
Resource: Code Execution
  - Operation: for loop
    - Iterate over the steps in the plan
      - Operation: match
        - Perform pattern matching on each step
      - Operation: Action::CodeToRO
        - If the step is of type Action::CodeToRO
          - Operation: calcualte_for_element
            - Calculate the constitution rule for the element and nodes
          - Operation: client.chat().create()
            - Create a chat message with the calculated request
          - Operation: write_new_self_content()
            - Write the new self content obtained from the response to the element
      - Operation: Action::FolderToRO
        - If the step is of type Action::FolderToRO
          - Operation: calcualte_for_element
            - Calculate the constitution rule for the element and nodes
          - Operation: client.chat().create()
            - Create a chat message with the calculated request
          - Operation: write_new_self_content()
            - Write the new self content obtained from the response to the element