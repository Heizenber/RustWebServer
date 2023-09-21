
mod items;
mod content_loader;

use actix_web::web;
use super::path::Path;

pub fn app_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {prefix: "/".to_string()};
    app.route(&base_path.define("".to_string()),
            web::get().to(items::items));
}