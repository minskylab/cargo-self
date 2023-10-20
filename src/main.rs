use std::path::PathBuf;

use async_openai::Client;
use cargo_self::engine::{
    constitution::ConstitutionDynamic,
    // model::{create_code_to_ro, create_folder_to_ro},
    planner::{Action, Plan},
};

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let root = PathBuf::from("./Cargo.toml");
    let constitution_name = "constitution.md".to_string();

    let plan = Plan::new(root);

    let constitution_rule = ConstitutionDynamic::new(constitution_name);

    let client = Client::new();

    let nodes = plan.nodes().clone();

    for step in plan {
        match step {
            Action::CodeToRO { element } => {
                println!("code to ro: {element:?}");

                let request = constitution_rule.calculate_for_element(&element, &nodes);

                // let req = create_code_to_ro(element.clone().content.unwrap());

                let res = client.chat().create(request).await.unwrap();

                let new_self_content = res
                    .choices
                    .first()
                    .unwrap()
                    .message
                    .content
                    .to_owned()
                    .unwrap();

                element.write_new_self_content(new_self_content);
            }
            Action::FolderToRO {
                element,
                children: _,
            } => {
                println!("folder to ro: {element:?}");
                // println!("neighbors: {:?}", children);

                let request = constitution_rule.calculate_for_element(&element, &nodes);

                let res = client.chat().create(request).await.unwrap();

                let new_self_content = res
                    .choices
                    .first()
                    .unwrap()
                    .message
                    .content
                    .to_owned()
                    .unwrap();

                element.write_new_self_content(new_self_content);
            }
        }
    }

    // Definitions:
    // - A "Resource" refers to crucial structures, entities, or data types within the code.
    // - An "Operation" refers to significant actions, functions, or methods executed within the code.

    // Guidelines for R&O Representation:
    // 1. Resources Identification:
    //    a. Library Imports: List the primary libraries or modules being imported.
    //    b. Input Filters: Catalog input structures or filters.
    //    c. Main Object: Identify the principal object, struct, or class.

    // 2. Operations Identification:
    //    a. Under the main object, struct, or class, list the associated operations.
    //    b. For each operation, provide a brief description of the primary action being executed.

    // 3. Structuring:
    //    a. Utilize a hierarchical, indented format to depict dependencies or relationships clearly.
    //    b. Ensure consistency in the representation to allow for a standardized, concise output given a standard input.

    // 4. Conciseness and Abstraction:
    //    a. Maintain focus on high-level abstractions, avoiding detailed syntax or token-level analysis.
    //    b. Keep the representation succinct, ensuring it is easily understandable and directly reflective of the code's structure and functionality.
}
