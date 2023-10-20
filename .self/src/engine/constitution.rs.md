Resource: Library Imports
  - async_openai, serde, std, super, handlebars

Resource: ConstitutionDynamic Struct
  Operation: new
    - Creates a new instance of ConstitutionDynamic struct with a provided name.
  Operation: constitution_filepath
    - Returns the filepath for the constitution based on the element and the struct's name.
  Operation: system_input_data
    - Returns a tuple containing the system part and input part of the constitution, given an element and nodes.
  Operation: calculate_for_element
    - Calculates the CreateChatCompletionRequest for a specific element and nodes.

Resource: ElementMinimized Struct
  - Fields: is_file, path, content, children

Resource: ConstitutionPayload Struct
  - Field: element

Operation: new
  - Creates a new instance of ConstitutionDynamic struct with a provided name.
Operation: constitution_filepath
  - Returns the filepath for the constitution based on the element and the struct's name.
Operation: system_input_data
  - Returns a tuple containing the system part and input part of the constitution, given an element and nodes.
Operation: calculate_for_element
  - Calculates the CreateChatCompletionRequest for a specific element and nodes.