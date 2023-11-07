use api_handlers::{
    ap_quote_handler::alan_partridge_quote_handler as ap_quote_handler,
    octopus_handler::octopus_handler,
};

use axum::{routing::get, Router};
use dotenv::dotenv;
use http::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let octopus_routes = Router::new()
        .layer(cors)
        .route("/", get(octopus_handler::main))
        .route("/about", get(octopus_handler::about))
        .route("/gas", get(octopus_handler::get_gas_consumption))
        .route(
            "/electric",
            get(octopus_handler::get_electricity_consumption),
        );

    let alan_partridge_quote_routes = Router::new().route("/", get(ap_quote_handler::main));

    let api_routes = Router::new()
        .nest("/octopus", octopus_routes)
        .nest("/alan-partridge-quote", alan_partridge_quote_routes);

    let app = Router::new().nest("/api", api_routes);

    axum::Server::bind(&"0.0.0.0:5001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
