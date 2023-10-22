Resource: Library Imports
- async_openai
- serde
- std
- super
- handlebars

Resource: ConstitutionDynamic Struct
Operation: new
- Create a new instance of ConstitutionDynamic struct

Operation: constitution_filepath
- Generate the file path associated with the ConstitutionDynamic struct

Operation: system_input_data
- Generate system and input data based on the ConstitutionDynamic struct and provided elements

Resource: ElementControlled Struct
- is_file: bool
- path: String
- content: Option<String>
- children: Vec<ElementControlled>

Resource: ConstitutionPayload Struct
- element: ElementControlled

Resource: SelfDynamic Trait
Operation: calculate
- Calculate the CreateChatCompletionRequest based on the ConstitutionDynamic struct, element, and project nodes