Resource: Library Imports
- std, async_openai, cargo_self, dotenv, tokio, fs, path

Resource: Module - `cli`

Resource: Module - `engine`

Resource: `main.rs` File
- Operation: main
  - Load environment variables from `.env` file
  - Define `root` as a `PathBuf` object with the path to `Cargo.toml`
  - Define `constitution_name` as a string with the value "constitution.md"
  - Create a new `JsonMemoryPersistence` object with a `PathBuf` to "output.json"
  - Create a new `Plan` object with `root` and `persistence`
  - Create a new `ConstitutionDynamic` object with `constitution_name`
  - Create a new `Client` object
  - Walk through elements of the `plan` using the `constitution_rule` and `client`
  - Consolidate the result
  - Write the consolidated content to a file named "output.yaml"