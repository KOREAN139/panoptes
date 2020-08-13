mod product;

use hyper_tls::HttpsConnector;
use hyper::{body::HttpBody as _, Client, Uri};
use tokio::io::{self, AsyncWriteExt as _};
use product::statuspage::Client as StatusPageClient;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let github: StatusPageClient = StatusPageClient::new(String::from("kctbh9vrtdwd"));
    let url: Uri = github.get_status_uri()
        .parse()
        .unwrap();

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut res = client.get(url).await?;
    println!("status: {}", res.status());

    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    Ok(())
}
