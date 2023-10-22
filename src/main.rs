use std::{fs, io, path::PathBuf};

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
    let database_name = PathBuf::from("output.json");

    let persistence = JsonMemoryPersistence::new(database_name);

    let mut plan = Plan::new(root, persistence);

    let dynamic = ConstitutionDynamic::new(constitution_name);
    let client = Client::new();

    let mut self_state = plan.process(&dynamic, &client).await;

    let consolidated_content = self_state.consolidate();

    fs::write("output.md", consolidated_content).unwrap();

    loop {
        let mut prompt = String::new();

        io::stdin().read_line(&mut prompt).unwrap();

        self_state = self_state.transmute(&client, prompt).await;
    }
}
