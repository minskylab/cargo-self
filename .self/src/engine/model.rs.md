Resource: Library Imports
  - async_openai.types

Resource: Operation: create_code_to_ro
  - Set up CreateChatCompletionRequestArgs with default values
  - Set max_tokens to 512
  - Set model to "gpt-3.5-turbo"
  - Set messages as an array of ChatCompletionRequestMessageArgs:
    - Set role to Role::System
    - Set content to DEFAULT_SYSTEM_PROMPT
    - Set role to Role::User
    - Set content to a formatted string that includes the source code parameter

Resource: Operation: create_folder_to_ro
  - Set up CreateChatCompletionRequestArgs with default values
  - Set max_tokens to 512
  - Set model to "gpt-3.5-turbo"
  - Set messages as an array of ChatCompletionRequestMessageArgs:
    - Set role to Role::System
    - Set content to DEFAULT_SYSTEM_PROMPT
    - Set role to Role::User
    - Set content to a formatted string that includes the sources joined with newlines