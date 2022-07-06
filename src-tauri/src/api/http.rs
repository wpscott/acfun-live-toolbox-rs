use hyper::{
    body::{aggregate, Buf},
    Body, Client, Method, Request,
};
use hyper_tls::HttpsConnector;

use serde::{de::DeserializeOwned, ser::Serialize};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn get_json<Json: DeserializeOwned>(url: &str) -> Result<Json, Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client.get(url.parse()?).await?;

    let body = aggregate(res).await?;

    let data: Json = serde_json::from_reader(body.reader())?;

    Ok(data)
}

pub async fn post_json<Data: Serialize, Json: DeserializeOwned>(
    url: &str,
    content_type: String,
    data: Data,
) -> Result<Json, Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("content-type", content_type)
        .body(Body::from(serde_json::to_string(&data)?))?;

    let res = client.request(req).await?;

    let body = aggregate(res).await?;

    let data: Json = serde_json::from_reader(body.reader())?;

    Ok(data)
}
