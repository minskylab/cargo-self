use std::{
    env,
    fs::{self, canonicalize},
    io::BufWriter,
    path::PathBuf,
};
// use walkdir::WalkDir;

use ignore::{Walk, WalkBuilder};

use cargo::{core::Shell, util::homedir};
// use cargo::ops::compile;
use cargo_self::engine::version::VERSION;

// use async_openai::{
//     types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
//     Client,
// };

// use cargo::{ops::compile, util::rustc};

#[tokio::main]
async fn main() {
    println!("Hello, Cargo Self v{}!", VERSION);

    let buf = BufWriter::new(Vec::new());

    let shell = Shell::from_write(Box::new(buf));
    let cwd = env::current_dir().unwrap();
    let homedir = homedir(&cwd).unwrap();

    let config = cargo::util::config::Config::new(shell, cwd, homedir);
    // let config = cargo::util::config::Config::default().unwrap();

    let manifest_path = canonicalize(PathBuf::from("./Cargo.toml")).unwrap();

    let ws = cargo::core::Workspace::new(&manifest_path, &config).unwrap();

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

    let root = ws.root();
    println!("root: {:?}", root);

    for entry in WalkBuilder::new(root).hidden(true).build() {
        let entry = entry.unwrap();

        let file_metadata = fs::metadata(entry.path()).unwrap();
        let modified = file_metadata.modified().unwrap();
        let file_type = file_metadata.file_type();

        let file_parent = entry.path().parent().unwrap();

        println!(
            "entry: {:?} | is file: {} | last modified: {} | parent: {:?}",
            entry.path(),
            file_type.is_file(),
            modified.elapsed().unwrap().as_secs(),
            file_parent,
        );
    }

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
