Resource: Library Imports
  - std, async_openai, cargo_self, tokio

Resource: Main Object
  - main

  Operation: Loop (For Each Step in Plan)
    - Description: Iterate through each step in the plan and execute the corresponding action.
    
    Operation: CodeToRO
      - Description: Convert code to R&O representation.
    
    Operation: FolderToRO
      - Description: Convert folder to R&O representation.