pub mod alan_partridge_quote_handler {
    use axum::Json;
    use rand::{thread_rng, Rng};
    use serde_derive::{Deserialize, Serialize};

    const QUOTE_URL: &str =
    "https://raw.githubusercontent.com/anontyro/alan-partridge-interface/quotes/data/quotes.json";

    #[derive(Serialize)]
    pub struct QuoteJsonOutput {
        quote: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct QuoteApi {
        quotes: Vec<String>,
    }

    pub async fn main() -> Json<QuoteJsonOutput> {
        let quote_list = get_quotes().await.unwrap();
        let quote_length = quote_list.quotes.len();

        let mut rng = thread_rng();
        let random_number = rng.gen_range(0..quote_length);

        let quote = quote_list.quotes[random_number].clone();

        let output = QuoteJsonOutput { quote };

        return Json(output);
    }

    async fn get_quotes() -> Result<QuoteApi, Box<dyn std::error::Error>> {
        let resp = reqwest::get(QUOTE_URL).await?.json::<QuoteApi>().await?;
        Ok(resp)
    }
}
