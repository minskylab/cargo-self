// use std::fs;

use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequest, CreateChatCompletionRequestArgs,
    Role,
};
use serde::{Deserialize, Serialize};
use std::fs;

use super::{dynamic::SelfDynamic, element::Element};
use handlebars::Handlebars;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstitutionDynamic {
    name: String,
    // reg: Registry<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ElementControlled {
    is_file: bool,
    path: String,
    content: Option<String>,
    children: Vec<ElementControlled>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConstitutionPayload {
    element: ElementControlled,
}

impl ConstitutionDynamic {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    fn constitution_filepath(&self, element: Element) -> String {
        let constitution_path = element.relative_path().with_file_name(self.name.clone());

        if constitution_path.exists() {
            return constitution_path.to_str().unwrap().to_string();
        }

        self.name.clone()
    }

    fn system_input_data(&self, element: &Element, nodes: &[Element]) -> (String, String) {
        let constitution_path = self.constitution_filepath(element.clone());

        // println!("constitution_path: {constitution_path:?}");

        let reg = Handlebars::new();

        let constitution_template_source = fs::read_to_string(constitution_path).unwrap();

        // println!("{constitution_template_source:?}");

        let constitution_rendered = reg
            .render_template(
                constitution_template_source.as_str(),
                &ConstitutionPayload {
                    element: ElementControlled {
                        is_file: element.is_file,
                        path: element.relative_path().to_str().unwrap().to_string(),
                        content: element.content.clone(),
                        children: element
                            .find_children(nodes)
                            .iter()
                            .map(|x| ElementControlled {
                                is_file: x.is_file,
                                path: x.relative_path().to_str().unwrap().to_string(),
                                content: x.content.clone(),
                                children: vec![],
                            })
                            .collect::<Vec<ElementControlled>>(),
                    },
                },
            )
            .unwrap();

        let mut splitted_constitution = constitution_rendered.split("# input\n");

        let system_part = splitted_constitution
            .nth(0)
            .unwrap()
            .split("# system\n")
            .collect::<Vec<&str>>()
            .join("")
            .trim()
            .to_string();

        let input_part = splitted_constitution
            .nth(0)
            .unwrap_or(&system_part)
            .split("# output\n")
            .nth(0)
            .unwrap_or(&system_part)
            .trim()
            .to_string();

        // println!("[SYSTEM] -- {system_part:?}\n[INPUT] -- {input_part:?}");

        (system_part, input_part)
    }
}

impl SelfDynamic for ConstitutionDynamic {
    fn calculate(
        &self,
        element: &Element,
        project_nodes: &[Element],
    ) -> CreateChatCompletionRequest {
        let (system_prompt, input_prompt) = self.system_input_data(element, project_nodes);

        let mut request = CreateChatCompletionRequestArgs::default();

        request
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
                    .content(input_prompt)
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap()
    }
}
