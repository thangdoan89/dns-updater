use std::error;
use std::net::IpAddr;

use dns_update::config::request::{Config, PathMethod};
use hyper;
use hyper_tls::HttpsConnector;
use serde_json::to_string;
use public_ip;

async fn get_public_ip() -> Option<IpAddr> {
    Some(public_ip::addr().await?)
}

async fn make_cloudflare_request(path_method: PathMethod, config: Config) -> Result<String, Box<dyn error::Error>> {

    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let comlete_uri: hyper::Uri = format!(
        "https://{uri}/{zone}/dns_records/{path}",
        uri=config.uri.clone(),
        zone=config.zone.clone(),
        path=config.record_id.clone(),
    ).parse().expect("Invalid URI");
    let req = hyper::Request::builder()
    .uri(comlete_uri)
    .header("X-Auth-Email", config.email.clone())
    .header("X-Auth-Key", config.api_key.clone())
    .method(path_method.method)
    .body(hyper::Body::from(path_method.body))?;

    let res = client.request(req).await?;

    if !res.status().is_success() {
        eprintln!("Request failed with status: {}", res.status());
    }

    // Get the response body bytes.
    let body_bytes = hyper::body::to_bytes(res.into_body()).await?;

    // Convert the body bytes to utf-8
    let json_value = String::from_utf8(body_bytes.to_vec()).unwrap();
    
    Ok(json_value)

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new();


    // Step 1: Get public IP
    let public_ip: IpAddr = get_public_ip().await.unwrap();

    // Step 2: Instantiate the PathMethod
    let update_record = PathMethod { body: format!(r#"{{
        "content": "{public_ip}",
        "ttl": {ttl}
    }}"#,
        public_ip=public_ip,
        ttl=config.ttl.clone()
    ).to_string(), method: hyper::Method::PATCH };

    // Step 3: Update the DNS Record
    let update_res = make_cloudflare_request(update_record, config).await?;

    println!("Successfully update the DNS Record:\n{}", to_string(&update_res).unwrap());

    Ok(())

}
