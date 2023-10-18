Resource: Main Object
    - VERSION: "0.1.4"
Resource: Library Imports
- mod: model
- mod: planner
- mod: prompts
- mod: version
Resource: Library Imports
  - std, async_graphql, chrono, uuid, crate

Resource: Input Filters
  - TaskFilter, MemberFilter, TeamFilter, ProjectFilter

Resource: ResourcesQuery Object
  Operation: tasks
    - Query tasks from database
  Operation: task_by_id
    - Query a specific task by ID from database
  Operation: members
    - Query members from database
  Operation: member_by_id
    - Query a specific member by ID from database
  Operation: member_by_email
    - Query a specific member by email from database
  Operation: projects
    - Query projects from database
  Operation: project_by_id
    - Query a specific project by ID from database
  Operation: teams
    - Query teams from database
  Operation: team_by_id
    - Query a specific team by ID from database
  Operation: labels
    - Query labels from database
  Operation: me
    - Query the authenticated member's data from database
  Operation: activity
    - Query activity logs from database with optional filters
Resource: Library Imports
- async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionResponse}
- cargo::{core::Shell, util::homedir}
- ignore::WalkBuilder
- serde::{Deserialize, Serialize}
- sha2::{Digest, Sha256}
- std::{collections::{HashMap, HashSet}, env, fmt::Debug, fs::{self, File}, io::{self, BufWriter}, path::{Path, PathBuf}}
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
- Find the children elements of the current element