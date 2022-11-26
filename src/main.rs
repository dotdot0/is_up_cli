#![allow(unused)]

use serde_derive::Deserialize;
use reqwest::Client;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about="Check whether a website is up or down")]
struct Cli{
    site_url: String
}

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

    let args = Cli::parse();

    let client: Response= Client::new()
        .get(format!("https://isitup.org/{}.json", args.site_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    
    if client.status_code == 1{
        println!("👍🏻 Up")    
    }
    else{
        println!("❌ Down")
    }
}
