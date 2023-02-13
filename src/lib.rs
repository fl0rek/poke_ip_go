use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod ip;
pub mod ip_api;
pub mod poke_api;
pub mod pokemon;
pub mod storage_api;

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchingPokemon {
    Pokemon(pokemon::Pokemon),
    FailedToLoad(pokemon::PokemonId),
    Loading(pokemon::PokemonId),
    Missing,
}

//pub type IpPokedex = HashMap<ip::Ip, (ip::IpDetails, MatchingPokemon)>;
pub type IpPokedex = Vec<(ip::IpDetails, MatchingPokemon)>;

fn seed_exists(poke: &IpPokedex, seed: pokemon::PokemonSeed) -> bool {
    for (_ip, poke) in poke {
        match poke {
            MatchingPokemon::Pokemon(p) => {
                if p.seed == seed {
                    return true;
                }
            }
            _ => (),
        }
    }
    return false;
}

/*
#[derive(PartialEq, Props)]
struct LocalIpProps {
    #[props(!optional)]
    my_ip: Option<String>,
}

#[allow(non_snake_case)]
fn LocalIp(cx: Scope<LocalIpProps>) -> Element {
    cx.render(match &cx.props.my_ip {
        Some(ip) => rsx!(input {
            readonly: true,
            value: "{ip}",
        }),
        None => rsx!(input {
            disabled: true,
            value: "Press \"Fetch\" to begin",
        }),
    })
}
*/

/*
#[derive(Props, Eq)]
struct CatchButtonProps {
    pending_pokemon: DiscoveredIp,
}
*/

/*
#[inline_props]
fn CatchButton<F>(cx: Scope, pending_pokemon: DiscoveredIp, onclick: F) -> Element
where
    F: FnMut(Event<MouseData>) -> (),
{
    cx.render(rsx! {
        button {
            class: "inline w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
            onclick: onclick,
            "Catch"
        }
    })
}
*/

#[derive(PartialEq, Eq)]
enum DiscoveredIp {
    Searching,
    Pending(pokemon::PokemonSeed),
    Found,
    Error(String),
}

pub fn app(cx: Scope) -> Element {
    let poke = storage_api::from_local_storage();
    let ip_pokedex = use_ref(&cx, || poke);
    let pending_pokemon = use_state(&cx, || DiscoveredIp::Searching);

    let poke_read = ip_pokedex.clone();
    let pending_poke_read = pending_pokemon.clone();
    cx.spawn(async move {
        //to_owned![pending_pokemon];
        match *pending_poke_read.current() {
            DiscoveredIp::Searching => {
                let maybe_ip = ip_api::fetch_ip().await.ok();
                let my_ip = maybe_ip.clone();
                log::info!("IP: {my_ip:#?}");

                let search_result = match maybe_ip
                    .map(|ip| (ip, ip::IpSource::Network))
                    .map(pokemon::PokemonSeed::from)
                {
                    Some(seed) => {
                        if seed_exists(&poke_read.read(), seed) {
                            DiscoveredIp::Found
                        } else {
                            DiscoveredIp::Pending(seed)
                        }
                        /*
                        match pokemon::Pokemon::new(seed).await {
                        Ok(p) => MatchingPokemon::Pokemon(p),
                        Err(e) => {
                        println!("FailedToLoad {seed:?}: {e}");
                        MatchingPokemon::FailedToLoad(seed.into())
                        }
                        }
                        */
                    }
                    None => DiscoveredIp::Error("Could not get IP".to_string()),
                };

                pending_poke_read.set(search_result);

                /*
                if let Some(ref my_ip) = my_ip {
                ip_pokedex
                .write()
                .push((ip::IpDetails::new_network(my_ip.clone()), maybe_pokemon));
                }
                */
            }
            _ => (),
        }
    });

    let update_ip = move |_evt| {
        to_owned![ip_pokedex, pending_pokemon];

        cx.spawn(async move {
            log::info!("I've been clicked");
            let maybe_ip = ip_api::fetch_ip().await.ok();
            let my_ip = maybe_ip.clone();
            log::info!("IP: {my_ip:#?}");

            let maybe_pokemon = match maybe_ip
                .map(|ip| (ip, ip::IpSource::Network))
                .map(pokemon::PokemonSeed::from)
            {
                Some(seed) => {
                    if seed_exists(&ip_pokedex.read(), seed) {
                        return;
                    }
                    match pokemon::Pokemon::new(seed).await {
                        Ok(p) => MatchingPokemon::Pokemon(p),
                        Err(e) => {
                            println!("FailedToLoad {seed:?}: {e}");
                            MatchingPokemon::FailedToLoad(seed.into())
                        }
                    }
                }
                None => MatchingPokemon::Missing,
            };

            if let Some(ref my_ip) = my_ip {
                ip_pokedex
                    .write()
                    .push((ip::IpDetails::new_network(my_ip.clone()), maybe_pokemon));
                pending_pokemon.set(DiscoveredIp::Found)
            }
        })
    };

    let update_random = move |_evt| {
        to_owned![ip_pokedex];
        let ip = ip::random_ip();
        cx.spawn(async move {
            let seed = pokemon::PokemonSeed::from((ip, ip::IpSource::Lootbox));
            if seed_exists(&ip_pokedex.read(), seed) {
                return;
            }
            let pokemon = match pokemon::Pokemon::new(seed).await {
                Ok(p) => MatchingPokemon::Pokemon(p),
                Err(e) => {
                    println!("FailedToLoad {seed:?}: {e}");
                    MatchingPokemon::FailedToLoad(seed.into())
                }
            };
            ip_pokedex
                .write()
                .push((ip::IpDetails::new_lootbox(), pokemon));
        })
    };

    let catch_button = match &*pending_pokemon.get() {
        DiscoveredIp::Pending(_poke) => {
            rsx! {
                button { class: "inline w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                    onclick: update_ip,
                    "Catch"
                }
            }
        }
        DiscoveredIp::Error(e) => {
            log::error!("Could not load poke to catch: {e}");
            rsx! {
                button { class: "inline w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-red-600 rounded transition duration-200",
                    disabled: true,
                    "Something went wrong"
                }
            }
        }
        DiscoveredIp::Searching => {
            rsx! {
                div { class: "flex justify-center items-center",
                    div { class: "spinner-border animate-spin inline-block w-8 h-8 border-4 rounded-full",
                        role: "status",
                        /*
                        span: { class: "visually-hidden",
                            "Loading..."
                        }
                        */
                    }
                }
            }
        }
        DiscoveredIp::Found => {
            rsx! {
                button { class: "inline w-full md:w-auto px-6 py-3 font-medium text-black bg-slate-300 rounded transition duration-200",
                    disabled: true,
                    "No more pokemon"
                }
            }
        }
    };

    let toggle_qr = move |_evt| {};

    cx.render(rsx!(
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        div {
            section { class: "antialiased bg-gray-100 text-gray-600 h-screen px-4",
                div { class: "m-2 w-full text-center mx-auto",
                    catch_button,
                    button {
                        class: "inline w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                        onclick: update_random,
                        "Lootbox"
                    }
                    button {
                        class: "inline w-full md:w-auto p-6 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                        onclick: toggle_qr,
                        "QR"
                    }
                },
                div { class: "flex flex-col justify-center h-full",
                    div { class: "w-full max-w-2xl mx-auto bg-white shadow-lg rounded-sm border border-gray-200",
                        header { class: "px-5 py-4 border-b border-gray-100",
                            h2 { class: "font-semibold text-gray-800",
                                "Catch history"
                            }
                        }
                        div { class: "p-3",
                            div { class: "overflow-x-auto",
                                table { class: "table-auto w-full",
                                    thead { class: "text-xs font-semibold uppercase text-gray-400 bg-gray-50",
                                        tr {
                                            th { class: "p-2 whitespace-nowrap",
                                                div { class: "font-semibold text-left",
                                                    "Name"
                                                }
                                            }
                                            th { class: "p-2 whitespace-nowrap",
                                                div { class: "font-semibold text-left",
                                                    "Seed"
                                                }
                                            }
                                            th { class: "p-2 whitespace-nowrap",
                                                div { class: "font-semibold text-left",
                                                    "Date caught"
                                                }
                                            }
                                        }
                                    }
                                    tbody { class: "text-sm divide-y divide-gray-100",
                                        {ip_pokedex.read().iter().map(|(ip_details, pokemon)|
                                            match &pokemon {
                                                MatchingPokemon::Pokemon(p) => {
                                                    rsx! {
                                                        tr {
                                                            td { class: "p-2 whitespace-nowrap",
                                                                pokemon::draw::Pokemon {
                                                                    name: p.name.clone(),
                                                                    //seed: p.seed.to_string(),
                                                                    sprite_url: p.sprite_url.clone()
                                                                }
                                                            }
                                                            td { class: "p-2 whitespace-nowrap text-slate-500",
                                                                div { class: "text-left",
                                                                    "{p.seed}"
                                                                }
                                                            }
                                                            td { class: "p-2 whitespace-nowrap text-slate-500",
                                                                div { class: "text-left",
                                                                    "{ip_details.timestamp}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                _ => unimplemented!("XXX0001"),
                                            }
                                        )}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            /*
            section { class: "py-8",
                div { class: "container px-4 mx-auto",
                    div { class: "p-4 mb-6 bg-white shadow rounded overflow-x-auto",
                        ul {
                            {ip_pokedex.read().iter().map(|(ip, (ip_details, pokemon))| {
                                log::debug!("l: {}", ip);
                                rsx! {
                                    li {
                                        "{ip}: {ip_details.date}"
                                    }
                                    {match &pokemon {
                                        MatchingPokemon::Pokemon(p) => {
                                            rsx!{
                                                crate::pokemon::draw::Pokemon {
                                                    name: p.name.clone(),
                                                    sprite_url: p.sprite_url.clone()
                                                }
                                            }
                                        },
                                        _ => unimplemented!("non happy poke path")
                                    }}
                                }
                            })}
                        }
                    }
                }
            }
            */
        }
    ))
}
