Resource: Library Imports
- std.path.PathBuf
- async_openai.Client
- cargo_self.engine.constitution.ConstitutionDynamic
- cargo_self.engine.planner.Action
- cargo_self.engine.planner.Plan
- tokio.runtime.Runtime

Main Object: Unknown

Operation: main
- Execute the main function
- Create a PathBuf object named `root` with the value `./Cargo.toml`
- Create a ConstitutionDynamic object named `constitution_rule` with the value "constitution.md"
- Create a Plan object named `plan` with the value of the `root` PathBuf
- Create a Client object named `client`
- Create a clone of `plan.nodes()` and assign it to `nodes`
- Iterate through each step in `plan`
  - If the step is `Action::CodeToRO`, execute the following:
    - Print the message "code to ro: {element:?}"
    - Calculate a request for the element using `constitution_rule` and `nodes`
    - If the request is successfully created, assign the content of `res.choices.first().unwrap().message.content` to `new_self_content`
    - Write `new_self_content` to the element's new self content
  - If the step is `Action::FolderToRO`, execute the following:
    - Print the message "folder to ro: {element:?}"
    - Calculate a request for the element using `constitution_rule` and `nodes`
    - If the request is successfully created, assign the content of `res.choices.first().unwrap().message.content` to `new_self_content`
    - Write `new_self_content` to the element's new self content