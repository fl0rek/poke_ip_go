use crate::ip::Ip;
use anyhow::anyhow;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize, Debug)]
pub struct IfconfigResponse {
    pub ip_addr: String,
    pub remote_host: String,
    //pub user_agent: Option<String>,
    //pub language: Option<String>,
    //pub referer: Option<String>,
    pub method: String,
    pub encoding: Option<String>,
    pub mime: String,
    pub via: String,
    pub forwarded: String,
}

const IFCONFIG_IP_URL: &str = "https://ifconfig.me/all.json";

#[cfg(not(target_family = "wasm"))]
pub async fn fetch_ip() -> anyhow::Result<Ip> {
    //return Ok(Ip::from((127, 0, 0, 1)));

    let body = reqwest::get(IFCONFIG_IP_URL).await?;
    let body_json = body.json::<IfconfigResponse>().await?;

    println!("ip info: {body_json:?}");

    if let IpAddr::V4(ip_v4addr) = body_json.ip_addr.parse()? {
        return Ok(ip_v4addr.into());
    }

    Err(anyhow!("unexpected ip addr"))
}

#[cfg(target_family = "wasm")]
pub async fn fetch_ip() -> anyhow::Result<Ip> {
    use serde_wasm_bindgen::from_value;
    use wasm_request::{get_options, request, Method};

    let op = get_options::<()>(&IFCONFIG_IP_URL, Method::Get, None, None);
    let body = request(op).await?;

    let response: IfconfigResponse = match from_value(body) {
        Ok(ip) => ip,
        Err(e) => return Err(anyhow!("invalid ip response: {e}")),
    };
    println!("poke: {response:?}");

    if let IpAddr::V4(ip_v4addr) = response.ip_addr.parse()? {
        return Ok(ip_v4addr.into());
    }

    Err(anyhow!("unexpected ip addr"))
}
