use actix_cors::Cors;
use actix_web::{get, middleware, route, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::{
    routes::ping::Rping,
    schemas::ping::{create_schema, Schema},
};

pub mod routes;
pub mod schemas;

/// Playground
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// Endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = std::sync::Arc::new(create_schema());

    let port = 8080;

    log::info!("Starting on Port: http://localhost:{}", port);
    log::info!("Playground running on: http://localhost:{}/graphiql", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .service(Rping)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
