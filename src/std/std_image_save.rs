extern crate base64;
extern crate image;

use hyper::{Client, Body, Method, Request, body};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageMaster {
    pub account_id: i32,
    pub image_base64: String,
    pub delete_flag: i8
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let base_img = image::open("/Users/satokeita/rust/test.jpg").unwrap();

    let mut buf:Vec<u8> = vec![];
    let _ = base_img.write_to(&mut buf, image::ImageOutputFormat::Png);

    let res_base64 = base64::encode(&buf);

    let request_image = ImageMaster {
        account_id: 1,
        image_base64: res_base64,
        delete_flag: 0
    };

    let body_string = serde_json::to_string(&request_image)?;

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:8000/image-save")
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