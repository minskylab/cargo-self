Resource: Library Imports
- async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionResponse}
- cargo::{core::Shell, util::homedir}
- ignore::WalkBuilder
- serde::{Deserialize, Serialize}
- sha2::{Digest, Sha256}
- std::{collections::{HashMap, HashSet}, env, fmt::Debug, fs::{canonicalize, metadata, File}, io::{self, BufWriter}, path::{Path, PathBuf}}


Resource: Plan Struct
Operation: new
- Initialize a new Plan struct with a provided root PathBuf object.

Operation: explore
- Explore the directory and collect a list of elements.
- Sort the elements based on their modification time and depth.
- Create new self files and add self paths for each file.
- Create a final elements list by adding the files and their corresponding parent directories in a reverse order.

Operation: analyze
- Perform analysis on the Plan struct.

Resource: Element Struct
Operation: write_new_self_content
- Write new self content to a file.
- Return true if the operation is successful.

Operation: relative_path
- Get the relative path of the element.

Operation: find_children
- Find the children elements of the current element.

Operation: find_parent
- Find the parent element of the current element.

Resource: Action Enum
- CodeToRO variant: Convert code element to Ro element.
- FolderToRO variant: Convert folder element to Ro element along with its children.

Resource: ActionResult Struct
- Contains information about the execution of an action.
- llm_executed: Whether the action was executed by the llm.
- llm_input: Optional input for llm execution.
- llm_result: Optional result of llm execution.

Resource: ComputationUnit Struct
- Contains information about a computation unit.
- action_executed: The action that was executed.
- result: Optional result of the computation.

Resource: AnalyzeResult Struct