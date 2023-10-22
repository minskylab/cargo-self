Resource: Library Imports
- async_openai, cargo, ignore, serde, sha2, std

Resource: Input Filters

Resource: Main Object
- Plan<Persistence>

    Operation: next
        - Get the next action from the plan

    Operation: new
        - Initialize a new Plan with a root path and persistence

    Operation: nodes
        - Get the nodes of the plan

    Operation: explore
        - Explore the elements in the root path and return a list of elements

    Operation: process
        - Process the plan by computing actions using the dynamic and client

Resource: Action
- CodeToRO
    
    Resource: Element
    Operation: element

- FolderToRO

    Resource: Element
    Operation: element