Resource: Library Imports
  - async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionResponse}
  - serde::{Deserialize, Serialize}
  - std::fmt::Debug
  - super::element::Element

Resource: ActionResult struct
  - llm_executed: bool
  - llm_input: Option<CreateChatCompletionRequest>
  - llm_result: Option<CreateChatCompletionResponse>

Resource: ComputationUnit struct
  - element: Element
  - result: Option<ActionResult>

Resource: Trait SelfStatePersistence
  - save(&self, result: &SelfState<Self>)
  - load(&self) -> Option<SelfState<Self>>

Resource: Impl ComputationUnit
  Operation: new
    - Create a new ComputationUnit with provided inputs
  Operation: new_without_llm
    - Create a new ComputationUnit without llm data

Resource: SelfState struct
  - computation_units: Vec<ComputationUnit>
  - persistence: Persistence

Resource: SelfState struct (Generic with Persistence)
  Operation: consolidate
    - Consolidate computation_unit elements into a single String
  Operation: transmute
    - Convert the current SelfState to another SelfState with a prompt

Note: The commented out code blocks have been excluded from the parsed representation.