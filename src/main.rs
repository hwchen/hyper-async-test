#![feature(async_await)]

use futures_util::TryStreamExt;
use hyper::Client;
use hyper_tls::HttpsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let url = match std::env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Please provide a url");
            return Ok(());
        }
    };

    let url = url.parse::<hyper::Uri>().unwrap();

    let res = fetch_url(url).await?;

    println!("{}", res);

    Ok(())
}

async fn fetch_url(url: hyper::Uri) -> Result<String> {
    let https = HttpsConnector::new(4)?;
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}", res.headers());

    let body = res.into_body();

    let bytes = body.try_concat().await?;

    Ok(
        String::from_utf8(bytes.to_vec())?
    )
}
