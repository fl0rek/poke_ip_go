use easy_storage::kv_storage::{self, KvStorage};

/*
#[cfg(not(target_family = "wasm"))]
pub fn from_local_storage() -> crate::IpPokedex {
    crate::IpPokedex::default()
}
*/

const SAVE_KEY: &str = "save";

/*
lazy_static! {
    ref const STORAGE: kv_storage::wasm_cookies_kv_storage::FileBasedKvStorage
}
*/

//#[cfg(target_family = "wasm")]
pub fn from_local_storage() -> crate::IpPokedex {
    #[cfg(target_family = "wasm")]
    let storage = kv_storage::wasm_cookies_kv_storage::WasmCookiesKvStorage::default();

    #[cfg(any(target_os = "windows", target_os = "android"))]
    let storage = kv_storage::file_based_kv_storage::FileBasedKvStorage::default();

    match storage.read(SAVE_KEY).as_deref() {
        Ok("") => {
            log::info!("No save data found, creating empty one");
            crate::IpPokedex::default()
        }
        Ok(save) => match serde_json::from_str(&save) {
            Ok(poke) => poke,
            Err(e) => {
                log::error!("Could not deserialize save data: {e}, creating empty one");
                crate::IpPokedex::default()
            }
        },
        Err(e) => {
            log::error!("Could not read save: {e}");
            crate::IpPokedex::default()
        }
    }
}

pub fn to_local_storage(pokedex: &crate::IpPokedex) {
    #[cfg(target_family = "wasm")]
    let storage = kv_storage::wasm_cookies_kv_storage::WasmCookiesKvStorage::default();

    #[cfg(any(target_os = "windows", target_os = "android"))]
    let storage = kv_storage::file_based_kv_storage::FileBasedKvStorage::default();

    let serialised = match serde_json::to_string(pokedex) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to serialise save data: {e}");
            return;
        }
    };

    match storage.write(SAVE_KEY, &serialised) {
        Ok(()) => (),
        Err(e) => log::error!("Failed to write save: {e}"),
    }
}
