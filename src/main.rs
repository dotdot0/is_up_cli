#![allow(unused)]

use serde_derive::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct Response{
    domain: String,
    port: i32,
    status_code: i32,
    response_ip: Option<String>,
    response_code: Option<i32>,
    response_time: f32
}

#[tokio::main]
async fn main() {

    let client: Response= Client::new()
                            .get("https://isitup.org/pratushrai.io.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
    println!("{:#?}", client);
}
