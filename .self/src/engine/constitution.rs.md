Resource: Library Imports
  - async_openai, serde, std, handlebars

Resource: ConstitutionDynamic Struct
  - name: String

Resource: ElementMinimized Struct
  - is_file: bool
  - path: String
  - content: String
  - children: Vec\<ElementMinimized\>

Resource: ConstitutionPayload Struct
  - element: ElementMinimized

Operation: new (Constructor)
  - Create a new instance of ConstitutionDynamic

Operation: constitution_filepath
  - Get the filepath of the constitution file for a given element

Operation: system_input_data
  - Generate system and input data for the constitution template rendering process

Operation: calculate_for_element
  - Calculate the chat completion request for a given element and nodes