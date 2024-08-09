use actix_web::{web, App, HttpServer, Responder};
mod graphql_schema;
mod handlers;
mod models;

async fn index() -> impl Responder {
    "Welcome to Foxtrot Backend"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .configure(handlers::init_routes)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
