use std::path::Path;

use cargo_self::engine::planner::Plan;

#[tokio::main]
async fn main() {
    let root = Path::new("./Cargo.toml");

    let plan = Plan::new(root);

    // plan.analyze();

    for step in plan {
        println!("step: {:?}", step);
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
    //     .messages([
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::System)
    //             .content("You are a helpful assistant.")
    //             .build()
    //             .unwrap(),
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::User)
    //             .content("Who won the world series in 2020?")
    //             .build()
    //             .unwrap(),
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::Assistant)
    //             .content("The Los Angeles Dodgers won the World Series in 2020.")
    //             .build()
    //             .unwrap(),
    //         ChatCompletionRequestMessageArgs::default()
    //             .role(Role::User)
    //             .content("Where was it played?")
    //             .build()
    //             .unwrap(),
    //     ])
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
