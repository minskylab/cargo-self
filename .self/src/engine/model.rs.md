Resource: Library Imports
- async_openai::types

Resource: Main Object
- create_folder_to_ro

Operation: create_folder_to_ro
- Create a chat completion request with the given element and children.
- Generate sources by iterating over children and formatting the relative path and content of each child.
- Set the maximum tokens to 512.
- Set the model to "gpt-3.5-turbo".
- Create system and user messages for the chat completion request.