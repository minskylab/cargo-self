Resource: Library Imports
  - async_openai
  - serde
  - std

Resource: ActionResult Struct
  - llm_executed: bool
  - llm_input: Option<CreateChatCompletionRequest>
  - llm_result: Option<CreateChatCompletionResponse>

Resource: ComputationUnit Struct
  - element: Element
  - result: Option<ActionResult>

Resource: SelfStatePersistence Trait
  - save(&self, result: &SelfState<Self>)
  - load(&self) -> Option<SelfState<Self>>

Resource: SelfState Struct
  - computation_units: Vec<ComputationUnit>
  - persistence: Persistence

Operation: ComputationUnit new
  - Creates a new ComputationUnit object with specified parameters

Operation: ComputationUnit new_without_llm
  - Creates a new ComputationUnit object without llm_result

Resource: SelfStatePersistence Trait
  - save(&self, result: &SelfState<Self>)
  - load(&self) -> Option<SelfState<Self>>

Operation: SelfState consolidate
  - Consolidates the computation units into a single string

Operation: SelfState transmute
  - Transmutes the SelfState object using OpenAI GPT-3.5 model