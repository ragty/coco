use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CocoTag {
    pub name: String,
    pub display_name: String,
    pub date: i64,
}

impl Default for CocoTag {
    fn default() -> Self {
        CocoTag {
            name: "".to_string(),
            display_name: "".to_string(),
            date: 0,
        }
    }
}