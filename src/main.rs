#![feature(async_await)]

use futures_util::TryStreamExt;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use serde_json;
use tracing::{
    info,
    trace,
    instrument,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // set up logging
    let subscriber = tracing_fmt::FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // instantiate client
    let https = HttpsConnector::new(4)?;
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    let url = "https://jsonplaceholder.typicode.com/users";

    let url = url.parse::<hyper::Uri>().unwrap();

    let res = fetch_json_url(url, &client).await?;

    info!("{:#?}", res);

    Ok(())
}

#[instrument]
async fn fetch_json_url<C>(url: hyper::Uri, client: &Client<C, hyper::Body>) -> Result<Vec<User>>
    where C: hyper::client::connect::Connect + 'static
{
    let res = client.get(url).await?;

    trace!(
        status = res.status().as_u16(),
        headers = &format!("{:?}", res.headers()).as_str(),
    );

    let bytes = res.into_body().try_concat().await?;

    let users = serde_json::from_slice(&bytes)?;

    Ok(users)
}

#[derive(Debug, Deserialize)]
struct User {
    id: i32,
    name: String,
}
