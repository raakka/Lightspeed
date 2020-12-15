///////////////////////////////////////////////////////////////////////////////////////////////////
// ACTIX APP MAIN

// USING CUSTOM MODULES
#[path = "config.rs"]
mod config;

#[path = "handlers.rs"] 
mod handlers;

use actix_web::{
    web,
    App,
    HttpServer,
    middleware
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // LOGGER
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // PULLING IN CONFIG STRUCT
    let cfg = crate::config::config::Config::from_env().unwrap();

    ////////////////////////////////////////////////// 
    // HTTP SERVER OBJECT
    // YUKI OKUSHI MAKES IT THIS WAY SO I WILL TOO
    let server = HttpServer::new( move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/routelol")
                     .route(web::post()
                            .to(handlers::handlers::redisfn)
                    )
            )
    })
    .bind(cfg.api_addr.clone())?
    .run();

    server.await
}
