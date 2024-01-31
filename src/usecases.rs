// usecases.rs
use crate::domain::Coin;
use crate::ports::CoinGecko;

pub fn get_coin_price(coin: Coin, api: &dyn CoinGecko) -> Result<String, ureq::Error> {
    api.get_price(&coin.name)
}