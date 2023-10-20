Resource: Library Imports
  - async_openai.types

Resource: Model Functions
  Operation: create_folder_to_ro
    - Create a chat completion request for generating code from a folder hierarchy.
    - Accepts an Element object and a vector of child Elements as input.
    - Returns a CreateChatCompletionRequest object.
    - Initializes a CreateChatCompletionRequestArgs object.
    - Constructs the sources string by iterating over the children and formatting their relative paths and self content.
    - Sets the maximum tokens to 512 and the model to "gpt-3.5-turbo" in the request.
    - Adds two messages to the request: a system message with the default system prompt and a user message with a prompt generated from the sources.
    - Returns the built CreateChatCompletionRequest object.