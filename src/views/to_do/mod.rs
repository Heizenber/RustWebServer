use actix_web::web;
mod create;
use super::path::Path;

pub fn item_factory(app: &mut web::ServiceConfig) {
    let base_path = Path {
        prefix: "/item".to_string(),
    };
    app.route(
        &base_path.define("/create/{title}".to_string()),
        web::get().to(create::create),
    );
}
