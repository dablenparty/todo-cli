use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct TodoItem {
    pub short_desc: String,
    pub long_desc: Option<String>,
    pub completed: bool,
    // TODO: due date? uuid?
}
