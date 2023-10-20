Resource: Library Imports
- async_openai::types

Resource: Main Object
- N/A

Operation: create_folder_to_ro
- Create a chat completion request to generate responses based on provided sources
- Inputs:
  - element: The element for which the folder is being created
  - children: The list of child elements in the folder
- Output:
  - CreateChatCompletionRequest object with specified arguments