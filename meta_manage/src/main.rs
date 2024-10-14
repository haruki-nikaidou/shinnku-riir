use serde::Deserialize;

mod search;

fn main() {
    println!("Hello, world!");
}


#[derive(Debug, Deserialize)]
pub struct SearchConfig {
    pub host: String,
    pub api_key: String,
}
