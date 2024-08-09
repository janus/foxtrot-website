use actix_web::{web, HttpResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(graphql_handler)));
}

async fn graphql_handler() -> HttpResponse {
    HttpResponse::Ok().body("GraphQL Endpoint")
}
