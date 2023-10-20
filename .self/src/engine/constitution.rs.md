use std::path::PathBuf;

use super::planner::Element;

pub struct ConstitutionTemplate {
    name: String,
}

impl ConstitutionTemplate {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    fn constitution_element_path(&self, element: Element) -> PathBuf {
        element.relative_path().join(self.name.clone())
    }
}
