use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{state::SelfState, state::SelfStatePersistence};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct JsonMemoryPersistence {
    path: PathBuf,
}

impl JsonMemoryPersistence {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl SelfStatePersistence for JsonMemoryPersistence {
    fn save(&self, result: &SelfState<Self>) {
        let output_file = std::fs::File::create(self.path.clone()).unwrap();
        serde_json::to_writer_pretty(output_file, &result).unwrap();
    }

    fn load(&self) -> Option<SelfState<Self>> {
        let input_file = std::fs::File::open(self.path.clone()).unwrap();
        serde_json::from_reader(input_file).ok()
    }
}
