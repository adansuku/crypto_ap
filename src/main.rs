// main.rs
mod domain;
mod usecases;
mod ports;

use domain::Coin;
use usecases::get_coin_price;
use ports::CoinGeckoImpl;
use std::io::stdin;

fn main() {
    let mut coin_name = String::new();
    println!("Which cryptocurrency do you want to check?");
    let _ = stdin()
        .read_line(&mut coin_name)
        .expect("An error occurred reading from stdin");

    let coin = Coin::new(&coin_name);
    let api = CoinGeckoImpl;
    let result_price = get_coin_price(coin, &api);
    match result_price {
        Ok(price) => println!("The price is: ${}", price),
        Err(error) => println!("Error fetching price: {}", error),
    }
}