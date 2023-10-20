Resource: Library Imports
  - std.path.PathBuf
  - async_openai::Client
  - cargo_self.engine.constitution.ConstitutionDynamic
  - cargo_self.engine.persistence.JsonMemoryPersistence
  - cargo_self.engine.planner.Plan
  - dotenv.dotenv
  - tokio.main()

Resource: src/lib.rs
  Resource: cli
  Resource: engine

Resource: src/cli

Resource: src/engine

Resource: src/main.rs
  Operation: main()
    - Load environment variables from .env file
    - Set the root path as "./Cargo.toml" as a PathBuf
    - Set the constitution_name as "constitution.md"
    - Initialize a JsonMemoryPersistence with the path "./output.json"
    - Initialize a new Plan with the root path and persistence
    - Create a new ConstitutionDynamic with the constitution_name
    - Create a new Client
    - Walk the elements in the plan using the constitution_rule and client
    - Create a new output_file as "output.json"
    - Write the result to the output_file as a pretty JSON using serde_json