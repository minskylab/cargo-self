Resource: Library Imports
  - clap

Resource: Main Object
  - Cli struct

Operation: Command Declaration
  - name: "cargo-self"
  - about: "Cargo extended with LLM"
  - long_about: None

Operation: Subcommand Declaration
  - enum: Commands
  - Subcommand: Run