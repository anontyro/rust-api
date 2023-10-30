use api_handlers:: {
    root_handler::root_handler as root_handler,
    ap_quote_handler::alan_partridge_quote_handler as ap_quote_handler,
    octopus_handler::octopus_handler as octopus_handler,
};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler::main))
        .route("/alan-partridge-quote", get(ap_quote_handler::main))
        .route("/octopus", get(octopus_handler::main))
        .route("/octopus/gas", get(octopus_handler::get_gas_consumption))
        .route("/octopus/electric", get(octopus_handler::get_electricity_consumption));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}