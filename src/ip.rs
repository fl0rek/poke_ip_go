use anyhow::anyhow;
use rand::prelude::*;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, SystemTime};

//pub type Ip = String;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Ip((u8, u8, u8, u8));

impl From<Ipv4Addr> for Ip {
    fn from(value: Ipv4Addr) -> Self {
        match value.octets() {
            [o0, o1, o2, o3] => Self((o0, o1, o2, o3)),
            //_ => panic!("wtf Invalid number of octets in ipv4"),
        }
    }
}

impl From<Ip> for u32 {
    fn from(value: Ip) -> Self {
        bytemuck::cast([value.0 .0, value.0 .1, value.0 .2, value.0 .3])
    }
}

impl std::fmt::Display for Ip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0 .0, self.0 .1, self.0 .2, self.0 .3)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum IpSource {
    Network,
    Lootbox,
}

pub fn random_ip() -> Ip {
    let mut rng = rand::thread_rng();
    Ip(rng.gen())
    //let ip: (u8, u8, u8, u8) = rng.gen();
    //format!("{}.{}.{}.{}", ip.0, ip.1, ip.2, ip.3)
}

pub struct IpDetails {
    pub date: String,
    pub source: IpSource,
}

impl IpDetails {
    pub fn new_network() -> Self {
        Self {
            source: IpSource::Network,
            ..Default::default()
        }
    }
    pub fn new_lootbox() -> Self {
        Self {
            source: IpSource::Lootbox,
            ..Default::default()
        }
    }
}

impl Default for IpDetails {
    fn default() -> Self {
        Self {
            date: format!("{:?}", SystemTime::now()),
            source: IpSource::Network,
        }
    }
}

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

pub async fn fetch_ip() -> anyhow::Result<Ip> {
    let body = reqwest::get(IFCONFIG_IP_URL).await?;
    let body_json = body.json::<IfconfigResponse>().await?;

    println!("ip info: {body_json:?}");

    if let IpAddr::V4(ip_v4addr) = body_json.ip_addr.parse()? {
        return Ok(ip_v4addr.into());
    }

    Err(anyhow!("unexpected ip addr"))
}
