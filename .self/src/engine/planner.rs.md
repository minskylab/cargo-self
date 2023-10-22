Resource: Library Imports
  - async_openai, cargo, colored, ignore, serde, sha2, std

Resource: Plan
  Operation: new
    - Initialize a new Plan with a root path and persistence object
    - Explore the root path and store the elements in nodes and nodes_buffer

  Operation: nodes
    - Get a reference to the nodes vector

  Operation: explore
    - Explore a directory and its children to collect file and directory elements
    - Sort the elements by modified time and depth
    - Separate files and directories
    - Calculate and assign self_path, self_content, and self_hash for files and directories

  Operation: process
    - Process the plan by executing actions on elements
    - Check if element was previously processed by comparing self_hash with last_state
    - Calculate the constitution using dynamic data, nodes, and the element
    - Make a request to the OpenAI API and update the self_content of the element
    - Create a ComputationUnit object with the element, request, response, and computed flag
    - Save the state

Resource: Action
  - CodeToRO
  - FolderToRO

Resource: Element
  - path
  - parent
  - modified
  - is_file
  - depth
  - content
  - self_path
  - self_content
  - self_hash

Resource: ComputationUnit
  - element
  - request
  - response
  - computed

Resource: SelfState
  - computation_units
  - persistence

Operation: add_extension
  - Add an extension to a given path