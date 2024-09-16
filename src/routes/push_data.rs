use actix_web::{ get, web, Responder };

use crate::config::setup::get_da_config;

#[get("/push_data/{data}")]
async fn push_data(data: web::Path<String>) -> impl Responder {
    let adapter = get_da_config();
    adapter.adapter_impl.push(b"shrihari", data.to_string().as_str()).await;

    format!("Hello, user with ID: {data}")
}
