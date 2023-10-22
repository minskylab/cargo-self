Resource: Library Imports
- std, async_openai, cargo_self, dotenv

Resource: Main Object
- Function: main
    - Calls dotenv function
    - Initializes variables: root, constitution_name, database_name
    - Creates JsonMemoryPersistence object
    - Creates Plan object
    - Creates ConstitutionDynamic object
    - Creates Client object
    - Executes plan.process function with dynamic and client objects
    - Consolidates self_state
    - Writes consolidated_content to output.md
    - Reads user input into prompt variable
    - Executes self_state.transmute function with client and prompt objects