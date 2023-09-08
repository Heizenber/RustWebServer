use actix_web::{web, Responder};
use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory};
use crate::json_serialization::to_do_items::ToDoItems;

pub async fn get() -> impl Responder {
    let state = read_file("./state.json");
    let mut array_buffer = vec![];

    for (key, value) in state {
        let item_type = value.to_string();
        let item = to_do_factory(&item_type, &key).unwrap();
        array_buffer.push(item);
    }
    let return_package: ToDoItems = ToDoItems::new(array_buffer);
    web::Json(return_package)
}