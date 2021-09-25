use hyper::{Client, Body, Method, Request, body};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub pw: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user = User {
        name: "test user 1".to_string(),
        pw: "abc".to_string()
    };

    let body_string = serde_json::to_string(&user)?;

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:8000/signup")
        .header("content-type", "application/json")
        .body(Body::from(body_string))?;

    let client = Client::new();
    let resp = client.request(req).await?;
    println!("Response: {}", resp.status());

    let request_body = resp.into_body();
    let bytes = body::to_bytes(request_body).await?;
    let body_stiring = String::from_utf8(bytes.to_vec()).unwrap();

    println!("{}", body_stiring);


    Ok(())
}