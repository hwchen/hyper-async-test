#![feature(async_await)]

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

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}", res.headers());

    let mut body = res.into_body();

    use std::io::Write;
    while let Some(next) = body.next().await {
        let chunk = next?;
        std::io::stdout().write_all(&chunk)?;
    }

    Ok(())
}
