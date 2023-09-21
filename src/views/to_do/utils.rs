use crate::json_serialization::to_do_items::ToDoItems;
use crate::state::read_file;
use crate::to_do::to_do_factory;
use actix_web::Responder;
use serde::{Deserialize, Serialize};

pub fn return_state() -> impl Responder + Serialize + 'static {
    let state = read_file("./state.json");
    let mut array_buffer = vec![];

    for (key, value) in state {
        let item_type = value.to_string().trim_matches('"').to_string();
        let item = to_do_factory(&item_type, &key.to_string()).unwrap();
        array_buffer.push(item);
    }
    let return_package: ToDoItems = ToDoItems::new(array_buffer);
    return_package
}
