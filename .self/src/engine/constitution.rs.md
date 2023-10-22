Resource: Library Imports
- async_openai, serde, std::fs, super, handlebars

Resource: ConstitutionDynamic Struct
- Property: name (String)

Resource: ElementMinimized Struct
- Property: is_file (bool)
- Property: path (String)
- Property: content (Option String)
- Property: children (Vec<ElementMinimized>)

Resource: ConstitutionPayload Struct
- Property: element (ElementMinimized)

Operation: ConstitutionDynamic.new
- Description: Create a new instance of ConstitutionDynamic struct with a specified name.

Operation: ConstitutionDynamic.constitution_filepath
- Description: Get the filepath for the constitution based on the provided element.

Operation: ConstitutionDynamic.system_input_data
- Description: Get the system and input data for the constitution based on the provided element and nodes.

Operation: ConstitutionDynamic.calculate (implements SelfDynamic trait)
- Description: Calculate and return a CreateChatCompletionRequest with prompts based on the provided element and project nodes.