use std::{fs, path::PathBuf};

use async_openai::Client;
use cargo_self::engine::{
    constitution::ConstitutionDynamic, persistence::JsonMemoryPersistence, planner::Plan,
};

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let root = PathBuf::from("./Cargo.toml");
    let constitution_name = "constitution.md".to_string();

    let persistence = JsonMemoryPersistence::new(PathBuf::from("output.json"));

    let mut plan = Plan::new(root, persistence);

    let constitution_rule = ConstitutionDynamic::new(constitution_name);

    let client = Client::new();

    let result = plan.walk_elements(&constitution_rule, &client).await;

    // for unit in result.computation_units.clone() {
    //     println!("Unit: {unit:?}");

    //     let self_content = unit.element.self_content();
    //     let content = self_content.get(0..10);

    //     // println!("Content: {content:?}");
    // }

    let consolidated_content = result.consolidate();

    fs::write("output.yaml", consolidated_content).unwrap();
}
