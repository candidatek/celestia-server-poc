use actix_web::{ App, HttpServer };

mod routes;
mod config;
mod adapters;

use config::setup::get_da_config;
use routes::init_routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let server_config = config::server::get_server_config();
    let adapter = get_da_config();

    HttpServer::new(|| App::new().configure(init_routes))
        .bind((server_config.host, server_config.port))?
        .workers(server_config.threads)
        .run().await
}
