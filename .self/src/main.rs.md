Resource: Library Imports
- std, async_openai, cargo_self, dotenv

Resource: Main Object
- main function

Operation: dotenv
- Load environment variables from .env file.

Operation: PathBuf::from
- Create a PathBuf object from a string.

Operation: JsonMemoryPersistence::new
- Create a new instance of JsonMemoryPersistence with a file path.

Operation: Plan::new
- Create a new instance of the Plan struct with a root path and persistence.

Operation: ConstitutionDynamic::new
- Create a new instance of ConstitutionDynamic with a constitution name.

Operation: Client::new
- Create a new instance of the Client struct.

Operation: plan.process
- Process the plan using the constitution rule and client.

Operation: self_state.consolidate
- Consolidate the state of the plan into a single content.

Operation: fs::write
- Write the consolidated content to a file.