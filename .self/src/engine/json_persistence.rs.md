Resource: Library Imports
  - std.path.PathBuf
  - serde.{Deserialize, Serialize}
  - super.state.SelfState
  - super.state.SelfStatePersistence

Resource: JsonMemoryPersistence Struct
  - Field: path (std.path.PathBuf)

Operation: new
  - Parameters:
    - path (std.path.PathBuf)
  - Action: Create a new instance of JsonMemoryPersistence with the provided path

Operation: save
  - Parameters:
    - result (&SelfState<Self>)
  - Action: Save the provided result to a file in JSON format

Operation: load
  - Action: Load the content of a JSON file and deserialize it into a SelfState<Self> object