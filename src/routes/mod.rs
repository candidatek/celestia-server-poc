use actix_web::web;
pub mod greet;
pub mod push_data;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(greet::greet_user);
    cfg.service(push_data::push_data);
}
