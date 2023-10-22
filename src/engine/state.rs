use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, CreateChatCompletionResponse, Role,
    },
    Client,
};

use serde::{Deserialize, Serialize};

use std::fmt::Debug;

use super::element::Element;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionResult {
    pub llm_executed: bool,
    pub llm_input: Option<CreateChatCompletionRequest>,
    pub llm_result: Option<CreateChatCompletionResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComputationUnit {
    pub element: Element,
    pub result: Option<ActionResult>,
}

pub trait SelfStatePersistence
where
    Self: Sized + Clone,
{
    fn save(&self, result: &SelfState<Self>);
    fn load(&self) -> Option<SelfState<Self>>;
}

impl ComputationUnit {
    pub fn new(
        element: Element,
        req: CreateChatCompletionRequest,
        res: CreateChatCompletionResponse,
        llm_executed: bool,
    ) -> Self {
        Self {
            element,
            result: Some(ActionResult {
                llm_executed,
                llm_input: Some(req),
                llm_result: Some(res),
            }),
        }
    }

    pub fn new_without_llm(element: Element) -> Self {
        Self {
            element,
            result: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfState<Persistence>
where
    Persistence: SelfStatePersistence + Clone,
{
    pub computation_units: Vec<ComputationUnit>,

    #[serde(skip)]
    pub persistence: Persistence,
}

impl<Persistence> SelfState<Persistence>
where
    Persistence: SelfStatePersistence + Clone,
{
    pub fn consolidate(&self) -> String {
        let mut consolidated = String::new();

        for unit in self.computation_units.clone() {
            let element = unit.element;

            let title = format!(
                "{} {}\n",
                "#".repeat(element.depth),
                element.relative_path().to_str().unwrap(),
            );

            let self_content = format!("{}{}\n", title, element.self_content());

            consolidated.push_str(&self_content);
        }

        consolidated
    }

    pub async fn transmute(
        &self,
        client: &Client<OpenAIConfig>,
        prompt: String,
    ) -> SelfState<Persistence> {
        let mut request = CreateChatCompletionRequestArgs::default();

        let consolidated = self.consolidate();
        let system_prompt = format!(
            "Your a programmer and the following is the description of a piece of software you are writing:\n\n{}\n\n.
            Based on the user input, return a list only bash commands, don't explain anything, get me only the predicted bash commands.", consolidated.as_str());

        let request = request
            .max_tokens(512u16)
            .model("gpt-3.5-turbo-16k")
            .messages([
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::System)
                    .content(system_prompt)
                    .build()
                    .unwrap(),
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(prompt)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        let response = client.chat().create(request).await.unwrap();

        let commands = response
            .choices
            .first()
            .unwrap()
            .message
            .content
            .to_owned()
            .unwrap();

        println!("commands: {commands}");

        SelfState {
            computation_units: self.computation_units.clone(),
            persistence: self.persistence.clone(),
        }
        // todo!()
    }
}
