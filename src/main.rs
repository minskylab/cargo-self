use std::path::PathBuf;

use async_openai::Client;
use cargo_self::engine::{
    model::create_new_default_request,
    planner::{Action, Plan},
};

#[tokio::main]
async fn main() {
    let root = PathBuf::from("./Cargo.toml");

    let plan = Plan::new(root);

    let client = Client::new();

    for step in plan {
        match step {
            Action::CodeToRO { element } => {
                // println!("code to ro: {:?}", element);
                let req = create_new_default_request(element.clone().content.unwrap());

                let res = client.chat().create(req).await.unwrap();

                let new_self_content = res
                    .choices
                    .first()
                    .unwrap()
                    .message
                    .content
                    .to_owned()
                    .unwrap();

                element.write_new_self_content(new_self_content);

                // println!("foo: {:?}", foo)
            }
            Action::FolderToRO { element } => {
                println!("folder to ro: {:?}", element);
            }
        }
    }

    // let options =
    //     cargo::ops::CompileOptions::new(&config, cargo::core::compiler::CompileMode::Build)
    //         .unwrap();

    // set example to named workspace member

    // println!("options.spec: {}", options.spec);

    // let client = Client::new();

    // let request = CreateChatCompletionRequestArgs::default()
    //     .max_tokens(512u16)
    //     .model("gpt-3.5-turbo")
    //     .messages()
    //     .build()
    //     .unwrap();

    // let response = client.chat().create(request).await.unwrap();

    // let package = ws.current().unwrap();

    // println!("package: {:?}", package.name());

    // package.targets().iter().for_each(|target| {
    //     println!("path: {:?}", target.src_path());

    //     println!("target: {:?}\n", target);
    // });

    // package.dependencies().iter().for_each(|dep| {
    //     println!("dep: {:?}\n", dep);
    // });

    // let res = compile(&ws, &options).unwrap();

    // println!("host: {}", res.host);
}
