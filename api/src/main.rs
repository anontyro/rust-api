use api_handlers::{
    ap_quote_handler::alan_partridge_quote_handler as ap_quote_handler,
    octopus_handler::octopus_handler, root_handler::root_handler,
};

use axum::{routing::get, Router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(root_handler::main))
        .route("/alan-partridge-quote", get(ap_quote_handler::main))
        .route("/octopus", get(octopus_handler::main))
        .route("/octopus/about", get(octopus_handler::about))
        .route("/octopus/gas", get(octopus_handler::get_gas_consumption))
        .route(
            "/octopus/electric",
            get(octopus_handler::get_electricity_consumption),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
