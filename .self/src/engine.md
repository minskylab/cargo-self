Resource: Library Imports
  - async_openai, types::CreateChatCompletionRequest
  - colored, cargo::core::Shell, util::homedir, cargo::util::config::Config, fs::canonicalize, metadata, cargo::core::Workspace
  - std::collections::{HashMap, HashSet}, env, fmt::Debug, fs::{File, BufWriter}, io::{self, BufRead}, path::{Path, PathBuf}, io::Result

Resource: Input Filters
  - None

Resource: Main Object
  - ConstitutionDynamic Object

Operation: new
  - Create a new ConstitutionDynamic object

Operation: constitution_filepath
  - Get the file path for the constitution template

Operation: system_input_data
  - Generate system and input data based on the constitution template and element data

Resource: ConstitutionPayload
  - Struct to hold the data for generating the constitution template

Operation: calculate
  - Generate a CreateChatCompletionRequest for a given element and project nodes