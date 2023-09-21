use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String,
}
