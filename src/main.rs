#![feature(async_await)]

use futures_util::TryStreamExt;
use hyper::Client;

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
    if url.scheme_part().map(|s| s.as_ref()) != Some("http") {
        println!("This doesn't support https right now");
        return Ok(());
    }

    let res = fetch_url(url).await?;

    println!("{}", res);

    Ok(())
}

async fn fetch_url(url: hyper::Uri) -> Result<String> {
    let client = Client::new();

    let res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}", res.headers());

    let body = res.into_body();

    let bytes = body.try_concat().await?;

    Ok(
        String::from_utf8(bytes.to_vec())?
    )
}
