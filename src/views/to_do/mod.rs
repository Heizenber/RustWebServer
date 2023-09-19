use actix_web::web;
mod create;
mod get;
use super::path::Path;
mod utils;
mod edit;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path = Path {
        prefix: "/item".to_string(),
    };
    app.route(
        &base_path.define("/create/{title}".to_string()),
        web::post().to(create::create),
    );
    app.route(
        &base_path.define("/get".to_string()),
        web::get().to(get::get),
    );
    app.route(
        &base_path.define("/edit".to_string()),
        web::put().to(edit::edit));
}
