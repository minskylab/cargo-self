Resource: Library Imports
  - async_openai::types
  - serde
  - std::fs
  - super::planner::Element
  - handlebars::Handlebars

Resource: ConstitutionDynamic struct
  Resource: ConstitutionDynamic::new() function
    - Create a new instance of ConstitutionDynamic struct
  Resource: constitution_filepath() function
    - Generate the file path for the constitution based on the element and name
  Resource: system_input_data() function
    - Retrieve system and input data for the constitution
  Resource: calculate_for_element() function
    - Calculate chat completion request for the given element and nodes

Resource: ElementMinimized struct

Resource: ConstitutionPayload struct

Operation: new
  - Create a new instance of ConstitutionDynamic struct
Operation: constitution_filepath
  - Generate the file path for the constitution based on the element and name
Operation: system_input_data
  - Retrieve system and input data for the constitution
Operation: calculate_for_element
  - Calculate chat completion request for the given element and nodes