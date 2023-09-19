use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String
}