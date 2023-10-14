Resource: Library Imports
    - async_openai.types

Resource: Prompts
    - DEFAULT_SYSTEM_PROMPT

Operation: create_new_default_request
    - Create a new chat completion request with default arguments
    - Set the maximum tokens to 512
    - Set the model to "gpt-3.5-turbo"
    - Create an array of messages
        - First message:
            - Set the role to system
            - Set the content to the default system prompt
        - Second message:
            - Set the role to user
            - Set the content to a formatted string containing the source code