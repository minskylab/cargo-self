use async_openai::types::CreateChatCompletionRequest;

use super::element::Element;

pub trait SelfDynamic {
    fn calculate(
        &self,
        element: &Element,
        project_nodes: &[Element],
    ) -> CreateChatCompletionRequest;
}
