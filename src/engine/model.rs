use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    Role,
};

use super::prompts::DEFAULT_SYSTEM_PROMPT;

pub fn create_new_default_request(source_code: String) -> CreateChatCompletionRequest {
    let mut request = CreateChatCompletionRequestArgs::default();

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
                    "give me only the output (in yaml format) of the code below:\n{}",
                    source_code
                ))
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap()
}
