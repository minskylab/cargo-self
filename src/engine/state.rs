use async_openai::types::{CreateChatCompletionRequest, CreateChatCompletionResponse};

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
    Self: Sized,
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
    Persistence: SelfStatePersistence,
{
    pub computation_units: Vec<ComputationUnit>,

    #[serde(skip)]
    pub persistence: Persistence,
}

// impl<Persistence> Default for SelfState<Persistence>
// where
//     Persistence: SelfStatePersistence<Persistence>,
// {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl<Persistence> SelfState<Persistence>
where
    Persistence: SelfStatePersistence,
{
    // pub fn new() -> Self {
    //     Self {
    //         computation_units: Vec::new(),
    //         // persistence:,
    //     }
    // }

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

    pub fn transmute(&self, prompt: String) -> SelfState<Persistence> {
        todo!()
    }
}
