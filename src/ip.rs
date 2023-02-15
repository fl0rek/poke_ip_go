use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Ip((u8, u8, u8, u8));

impl From<Ipv4Addr> for Ip {
    fn from(value: Ipv4Addr) -> Self {
        match value.octets() {
            [o0, o1, o2, o3] => Self((o0, o1, o2, o3)),
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

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum IpSource {
    Network,
    Lootbox,
}

pub fn random_ip() -> Ip {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    Ip(rng.gen())
}

#[derive(Serialize, Deserialize)]
pub struct IpDetails {
    pub ip: Ip,
    pub timestamp: f64,
    pub source: IpSource,
}

impl IpDetails {
    pub fn new_network(ip: Ip) -> Self {
        Self {
            ip,
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

/*
#[cfg(target_family = "wasm")]
fn get_unix_timestamp() -> u64 {
    let now = web_sys::DateTimeValue::now
}
*/

impl Default for IpDetails {
    fn default() -> Self {
        let now = instant::SystemTime::now();
        let ts = now
            .duration_since(instant::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Self {
            ip: Ip((127, 0, 0, 1)),
            timestamp: ts,
            source: IpSource::Network,
        }
    }
}
