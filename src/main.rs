#![feature(async_await)]

use futures_util::TryStreamExt;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://jsonplaceholder.typicode.com/users";

    let url = url.parse::<hyper::Uri>().unwrap();

    let res = fetch_json_url(url).await?;

    println!("{:#?}", res);

    Ok(())
}

async fn fetch_json_url(url: hyper::Uri) -> Result<Vec<User>> {
    let https = HttpsConnector::new(4)?;
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}", res.headers());

    let bytes = res.into_body().try_concat().await?;

    let users = serde_json::from_slice(&bytes)?;

    Ok(
        users
    )
}

#[derive(Debug, Deserialize)]
struct User {
    id: i32,
    name: String,
}
