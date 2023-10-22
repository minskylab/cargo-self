use std::{fs, path::PathBuf};

use async_openai::Client;
use cargo_self::engine::{
    constitution::ConstitutionDynamic, json_persistence::JsonMemoryPersistence, planner::Plan,
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

    let self_state = plan.process(&constitution_rule, &client).await;

    let consolidated_content = self_state.consolidate();

    fs::write("output.md", consolidated_content).unwrap();
}
