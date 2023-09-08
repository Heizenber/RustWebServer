use actix_web::{web, Responder};
use serde_json::{Map, value::Value};
use crate::state::read_file;

pub async fn get() -> impl Responder {
    let state = read_file("./state.json");
    web::Json(state)
}