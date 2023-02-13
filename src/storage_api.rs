use anyhow::anyhow;

#[cfg(not(target_family = "wasm"))]
pub fn from_local_storage() -> crate::IpPokedex {
    crate::IpPokedex::default()
}

const SAVE_KEY: &str = "save";

#[cfg(target_family = "wasm")]
pub fn from_local_storage() -> crate::IpPokedex {
    use wasm_cookies::{get, set, CookieOptions};

    /*
    fn inner() -> anyhow::Result<IpPokedex> {
        let serialised = get(SAVE_KEY).ok_or_else(|| anyhow!("No save data found"));
    }
    //wasm_cookies::set("key", "value", &wasm_cookies::CookieOptions::default());
    let
    */

    log::info!("{:#?}", wasm_cookies::all());

    crate::IpPokedex::default()
}
