Resource: Library Imports
  - std, async_openai, cargo_self, dotenv

Resource: Main Object
  - main function

Operation: main function
  - Load environment variables from `.env` file
  - Set root path to `./Cargo.toml`
  - Set constitution name to `constitution.md`
  - Create a new instance of `JsonMemoryPersistence` with output file path `output.json`
  - Create a new instance of `Plan` with root path and persistence
  - Create `ConstitutionDynamic` with constitution name
  - Create a new instance of `Client`
  - Process the plan using the constitution rule and client
  - Consolidate the content in self state
  - Write the consolidated content to `output.md` file