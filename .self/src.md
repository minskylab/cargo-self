Resource: Library Imports
    - std::path::PathBuf
    - async_openai::Client
    - cargo_self::engine::constitution::ConstitutionDynamic
    - cargo_self::engine::planner::{Action, Plan}
    - dotenv::dotenv
    - tokio::main

Resource: Main Object
    Operation: main
        - Define the main function
        - Load environment variables from .env file
        - Set the root path to "./Cargo.toml"
        - Set the constitution name to "constitution.md"
        - Create a new plan with the root path
        - Create a new constitution rule with the constitution name
        - Create a new OpenAI client
        - Retrieve the nodes from the plan
        - Iterate over each step in the plan
            - For CodeToRO action:
                - Print the element
                - Calculate the request for the element using the constitution rule and nodes
                - Send a chat create request to the OpenAI client and store the response
                - Get the new self content from the response
                - Write the new self content to the element
            - For FolderToRO action:
                - Print the element
                - Calculate the request for the element using the constitution rule and nodes
                - Send a chat create request to the OpenAI client and store the response
                - Get the new self content from the response
                - Write the new self content to the element