use actix_web::HttpRequest;
use actix_web::HttpResponse;
use serde_json::{value::Value, Map};
use super::utils::return_state;
use crate::processes::process_input;
use crate::state::read_file;
use crate::to_do;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let state: Map<String, Value> = read_file("./state.json");

    let title = req.match_info().get("title").unwrap().replace("\"", "");
    let title_reference = title.clone();
    let item = to_do::to_do_factory("pending", &title).expect("create");
    process_input(item, "create".to_string(), &state);
    // format!("{} created", title_reference)
    return HttpResponse::Ok().json(return_state());
}
