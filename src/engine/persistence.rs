use std::path::PathBuf;

use super::planner::{AnalyzeResult, SelfStatePersistence};

pub struct JsonMemoryPersistence {
    path: PathBuf,
}

impl JsonMemoryPersistence {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl SelfStatePersistence for JsonMemoryPersistence {
    fn save(&self, result: &AnalyzeResult) {
        let output_file = std::fs::File::create(self.path.clone()).unwrap();
        serde_json::to_writer_pretty(output_file, &result).unwrap();
    }

    fn load(&self) -> Option<AnalyzeResult> {
        let input_file = std::fs::File::open(self.path.clone()).unwrap();
        serde_json::from_reader(input_file).ok()
    }
}
