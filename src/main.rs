use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use structopt::StructOpt;

mod config;
mod moralis;
mod command;
mod error;

use command::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let opt = Opt::from_args();
    
    let env = config::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .route("moralis/get_balance", web::get().to(moralis::get_balance))     
            .app_data(Data::new(env.clone()))       
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .run()
    .await
}
