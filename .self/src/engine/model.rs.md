Resource: Library Imports
  - async_openai.types

Resource: Other Resources
  - Element
  - prompts.DEFAULT_SYSTEM_PROMPT

Operation: create_code_to_ro
  - Create a chat completion request for generating code output
  - Set the request parameters:
    - Maximum tokens: 512
    - Model: "gpt-3.5-turbo"
    - Messages:
      - Role: System, Content: DEFAULT_SYSTEM_PROMPT
      - Role: User, Content: "give me only the output (in plain yaml format, don't use yaml code box syntax, only a parsable yaml result) of the code below: [source_code value]"

Operation: create_folder_to_ro
  - Create a chat completion request for generating code output based on a folder and its children
  - Set the request parameters:
    - Maximum tokens: 512
    - Model: "gpt-3.5-turbo"
    - Messages:
      - Role: System, Content: DEFAULT_SYSTEM_PROMPT
      - Role: User, Content: "give me only the output (in plain yaml format, don't use yaml code box syntax, only a parsable yaml result) of the code below: [sources joined by newlines]"