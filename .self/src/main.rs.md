Resource: Library Imports
    - std, async_openai, cargo_self, dotenv, tokio

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: N/A

Resource: Main Object
    Operation: main
        - Load environment variables from .env file
        - Set "root" variable to a relative path
        - Set "constitution_name" variable to a string
        - Create a new Plan object with "root" path
        - Create a new ConstitutionDynamic object with "constitution_name"
        - Create a new Client object
        - Clone the nodes from plan into "nodes" variable
        - Iterate over each step in plan
            - If step is Action::CodeToRO
                - Print the element
                - Calculate request using constitution_rule and nodes
                - Call the create function on client's chat object with the request and assign the result to "res"
                - Get the content of the message from "res" and assign it to "new_self_content"
                - Write "new_self_content" to the element's new_self_content
            - If step is Action::FolderToRO
                - Print the element
                - Calculate request using constitution_rule and nodes
                - Call the create function on client's chat object with the request and assign the result to "res"
                - Get the content of the message from "res" and assign it to "new_self_content"
                - Write "new_self_content" to the element's new_self_content