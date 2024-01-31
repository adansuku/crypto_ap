use serde::Deserialize;

// Define the structures to deserialize the JSON response
#[derive(Deserialize)]
struct CoinData {
    market_data: MarketData,
}

#[derive(Deserialize)]
struct MarketData {
    current_price: CurrentPrice,
}

#[derive(Deserialize)]
struct CurrentPrice {
    usd: f64,
}

// ports.rs
pub trait CoinGecko {
    fn get_price(&self, coin: &str) -> Result<String, ureq::Error>;
}

pub struct CoinGeckoImpl;

impl CoinGecko for CoinGeckoImpl {
    fn get_price(&self, coin: &str) -> Result<String, ureq::Error> {
        let body: String = ureq::get(&format!(
            "https://api.coingecko.com/api/v3/coins/{}?localization=false",
            coin
        ))
        .call()?
        .into_string()?;
        
        // Deserialize the JSON response into an instance of CoinData
        let coin_data: CoinData = serde_json::from_str(&body).unwrap();
        
        // Extract the price in USD and convert it to a string
        Ok(coin_data.market_data.current_price.usd.to_string())
    }
}