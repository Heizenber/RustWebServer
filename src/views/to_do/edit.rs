use actix_web::{web, HttpResponse};
use serde_json::{Map, value::Value};

use super::utils::return_state;
use crate::state::read_file;

use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");
    let title_reference: &String = &to_do_item.title.clone();
    let title: String = to_do_item.title.clone();

    let status: String;
    match &state.get(title_reference) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            return HttpResponse::NotFound().json(
                format!("{} not in state", title_reference)
            )
        }
    }
    if &status == &to_do_item.status {
        return HttpResponse::Ok().json(return_state())
    }

    match to_do_factory(&status, title.as_str()) {
        Err(_item) => return HttpResponse::BadRequest().json(
            format!("{} not accepted", status)
        ),
        Ok(item) => process_input(item, "edit".to_string(), &state)
    }
    return HttpResponse::Ok().json(return_state())
}