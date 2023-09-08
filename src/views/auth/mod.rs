use actix_web::web;
mod login;
mod logout;
use super::path::Path;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: "/auth".to_string(),
    };
    let app = app.route(
        &base_path.define(String::from("/login")),
        web::get().to(login::login),
    );
}
