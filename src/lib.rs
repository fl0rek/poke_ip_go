use std::collections::HashMap;

pub mod ip;
pub mod pokemon;

pub enum MatchingPokemon {
    Pokemon(pokemon::Pokemon),
    FailedToLoad(pokemon::PokemonId),
    Loading(pokemon::PokemonId),
    Missing,
}

pub type IpPokedex = HashMap<ip::Ip, (ip::IpDetails, MatchingPokemon)>;

use crate::ip::{Ip, IpDetails, IpSource};
use crate::pokemon::PokemonId;


use dioxus::prelude::*;
use rustemon::{client::RustemonClient};

use lazy_static::lazy_static;

    lazy_static! {
        static ref RUSTEMON_CLIENT: RustemonClient = RustemonClient::default();
    }

#[derive(PartialEq, Props)]
struct LocalIpProps {
    #[props(!optional)]
    my_ip: Option<String>,
}

fn LocalIp(cx: Scope<LocalIpProps>) -> Element {
    cx.render(match &cx.props.my_ip {
        Some(ip) => rsx!(input {
            //readonly,
            value: "{ip}",
        }),
        None => rsx!(input {
            disabled: true,
            value: "Press \"Fetch\" to begin",
        }),
    })
}

fn app(cx: Scope) -> Element {
    let ip = use_state(&cx, || None);
    let ip_pokedex = use_ref(&cx, || crate::IpPokedex::default());

    let update_ip = move |_evt| {
        let ip_setter = ip.setter();
        to_owned![ip_pokedex];

        cx.spawn(async move {
            println!("I've been clicked");
            let maybe_ip = ip::fetch_ip().await.ok();
            let my_ip = maybe_ip.clone();
            println!("IP: {my_ip:#?}");
            //let pokemon_id = pokemon::PokemonId::from(my_ip);
            let maybe_pokemon = match maybe_ip.map(crate::pokemon::PokemonId::from) {
                Some(id) => match crate::pokemon::Pokemon::new(&RUSTEMON_CLIENT, id).await {
                    Ok(p) => MatchingPokemon::Pokemon(p),
                    Err(e) => {
                      println!("FailedToLoad {id:?}: {e}");
                      MatchingPokemon::FailedToLoad(id)
                    }
                },
                None => MatchingPokemon::Missing,
            };



            if let Some(ref my_ip) = my_ip {
                ip_pokedex
                    .write()
                    .insert(my_ip.clone(), (IpDetails::new_network(), maybe_pokemon));
            }
            ip_setter(my_ip);
        })
    };

    let update_random = move |_evt| {
        to_owned![ip_pokedex];
        let ip = ip::random_ip();
        cx.spawn(async move {
            let id = crate::pokemon::PokemonId::from(ip);
        let pokemon = match crate::pokemon::Pokemon::new(&RUSTEMON_CLIENT, id).await {
            Ok(p) => MatchingPokemon::Pokemon(p),
            Err(e) => {
                println!("FailedToLoad {id:?}: {e}");
                MatchingPokemon::FailedToLoad(id)
            }

        };
        ip_pokedex.write().insert(ip, (IpDetails::new_lootbox(), pokemon));
        })
    };

    cx.render(rsx!(
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        div {
            div { class: "py-8 px-6",
                div { class: "container px-4 mx-auto",
                    h2 { class: "text-2xl font-bold", "Fetch your IP from network" }
                    button {
                        class: "inline-block w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                        onclick: update_ip,
                        "Fetch"
                    }
                }

                div { class: "container px-4 mx-auto",
                    h2 { class: "text-2x1 font-bold", "Fetch random IP"}
                    button {
                        class: "inline-block w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                        onclick: update_random,
                        "Roll"
                    }
                }
            },

            LocalIp { my_ip: ip.get().as_ref().map(|ip| ip.to_string()) } 

            section { class: "py-8",
                div { class: "container px-4 mx-auto",
                    div { class: "p-4 mb-6 bg-white shadow rounded overflow-x-auto",
                        ul { 
                            {ip_pokedex.read().iter().map(|(ip, details)| {
                                println!("l: {}", ip);
                                rsx! { 
                                    li { 
                                        "{ip}: {details.0.date}" 
                                    } 
                                    {match &details.1 {
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
        }
    ))
}

