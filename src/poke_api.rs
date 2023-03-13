#[cfg(not(target_family = "wasm"))]
use rustemon::{client::RustemonClient, model::pokemon};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PokemonNames {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    pub sprites: PokemonSprites,
}

#[derive(Debug, Deserialize)]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub back_default: Option<String>,
}

#[cfg(not(target_family = "wasm"))]
lazy_static::lazy_static! {
    static ref RUSTEMON_CLIENT: RustemonClient = RustemonClient::default();
}

#[cfg(not(target_family = "wasm"))]
impl From<pokemon::PokemonSprites> for PokemonSprites {
    fn from(value: pokemon::PokemonSprites) -> Self {
        Self {
            front_default: value.front_default,
            back_default: value.back_default,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<pokemon::Pokemon> for Pokemon {
    fn from(value: pokemon::Pokemon) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sprites: value.sprites.into(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
pub async fn get_by_id(id: i64) -> anyhow::Result<Pokemon> {
    rustemon::pokemon::pokemon::get_by_id(id, &RUSTEMON_CLIENT)
        .await
        .map(Pokemon::from)
        .map_err(Into::into)
}

#[cfg(target_family = "wasm")]
pub async fn get_by_id(id: i64) -> anyhow::Result<Pokemon> {
    use anyhow::anyhow;
    use serde_wasm_bindgen::from_value;
    use wasm_request::{get_options, request, Method};

    let poke_url = format!("https://pokeapi.co/api/v2/pokemon/{id}");

    let op = get_options::<()>(&poke_url, Method::Get, None, None);
    let body = request(op).await?;

    log::info!("Response: {body:?}");

    match from_value(body) {
        Ok(poke) => {
            log::info!("got poke: {poke:?}");
            Ok(poke)
        }
        Err(e) => Err(anyhow!("Could not get poke {id}: {e}")),
    }
}
