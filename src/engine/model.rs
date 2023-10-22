use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    Role,
};

use super::{element::Element, prompts::DEFAULT_SYSTEM_PROMPT};

pub fn create_folder_to_ro(
    _element: Element,
    children: Vec<Element>,
) -> CreateChatCompletionRequest {
    let mut request = CreateChatCompletionRequestArgs::default();

    let sources = children
        .iter()
        .map(|child| {
            format!(
                "{}:\n{}",
                child.relative_path().to_str().unwrap(),
                child.self_content().clone(),
            )
        })
        .collect::<Vec<String>>();

    request
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(DEFAULT_SYSTEM_PROMPT)
                .build()
                .unwrap(),
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(format!(
                    "give me only the output (in plain yaml format, don't use yaml code box syntax, only a parsable yaml result) of the code below:\n{}",
                    sources.join("\n")
                ))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}
