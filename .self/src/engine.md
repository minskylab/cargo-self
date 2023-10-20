Resource: Library Imports
  - async_openai, cargo, handlebars, serde, sha2, std

Resource: Input Filters
  - None

Resource: main.rs
  Operation: VERSION
    - Constant string: "0.1.4"

Resource: prompts.rs
  Operation: DEFAULT_SYSTEM_PROMPT
    - Constant string: "Your task is to create a simplified Resources and Operations (R&O) representation..."

Resource: mod.rs
  Operation: constitution Module
  Operation: model Module
  Operation: planner Module
  Operation: prompts Module
  Operation: version Module

Resource: model.rs
  Operation: create_folder_to_ro
    - Convert an Element and its children to a CreateChatCompletionRequest structure

Resource: planner.rs
  Resource: Plan Struct
    Operation: Iterator.next
      - Get the next Action to be executed in the plan
  Resource: Element Struct
    Operation: write_new_self_content
      - Write new content to the Element file
    Operation: relative_path
      - Get the relative path of the Element
    Operation: find_children
      - Find the children Elements of the Element
    Operation: find_parent
      - Find the parent Element of the Element
    Operation: is_file
      - Check if the Element represents a file
    Operation: content
      - Get the content of the Element

Resource: constitution.rs
  Resource: ConstitutionDynamic Struct
    Operation: new
      - Create a new ConstitutionDynamic instance
    Operation: constitution_filepath
      - Get the filepath of the Constitution for a given Element
    Operation: system_input_data
      - Get the system prompt and input data for a given Element
    Operation: calculate_for_element
      - Calculate the CreateChatCompletionRequest for a given Element