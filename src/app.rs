use crate::ip;
use crate::ip_api;
use crate::pokedex::MatchingPokemon;
use crate::pokemon;
use crate::storage_api;
use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
enum DiscoveredIp {
    Searching,
    Pending(pokemon::PokemonSeed),
    Found,
    Error(String),
}

pub fn app(cx: Scope) -> Element {
    log::info!("Hello, World from Dixous");

    let poke = storage_api::from_local_storage();
    let ip_pokedex = use_ref(&cx, || poke);
    let pending_pokemon = use_state(&cx, || DiscoveredIp::Searching);

    // fix ownership across the XXX
    let ip_pokedex_read = ip_pokedex.clone();
    let pending_pokemon_read = pending_pokemon.clone();

    cx.spawn(async move {
        //to_owned![pending_pokemon];
        match *pending_pokemon_read.current() {
            DiscoveredIp::Searching => {
                let maybe_ip = ip_api::fetch_ip().await.ok();
                let my_ip = maybe_ip.clone();
                log::info!("IP: {my_ip:?}");

                let search_result = match maybe_ip
                    .map(|ip| (ip, ip::IpSource::Network))
                    .map(pokemon::PokemonSeed::from)
                {
                    Some(seed) => {
                        if ip_pokedex_read.read().seed_exists(seed) {
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

                pending_pokemon_read.set(search_result);

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
            log::info!("IP: {my_ip:?}");

            let maybe_pokemon = match maybe_ip
                .map(|ip| (ip, ip::IpSource::Network))
                .map(pokemon::PokemonSeed::from)
            {
                Some(seed) => {
                    if ip_pokedex.read().seed_exists(seed) {
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
                    .catch_pokemon(ip::IpDetails::new_network(my_ip.clone()), maybe_pokemon);
                pending_pokemon.set(DiscoveredIp::Found);
                storage_api::to_local_storage(&*ip_pokedex.read());
            }
        })
    };

    let update_random = move |_evt| {
        to_owned![ip_pokedex];
        let ip = ip::random_ip();
        cx.spawn(async move {
            let seed = pokemon::PokemonSeed::from((ip, ip::IpSource::Lootbox));
            if ip_pokedex.read().seed_exists(seed) {
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
                .catch_pokemon(ip::IpDetails::new_lootbox(), pokemon);
            storage_api::to_local_storage(&*ip_pokedex.read());
        })
    };

    let catch_button = match &*pending_pokemon.get() {
        DiscoveredIp::Pending(_poke) => {
            rsx! {
                button {
                    class: "block w-full",
                    onclick: update_ip,
                    div { class: "p-2.5 mt-3 flex items-center rounded-md px-4 duration-200 cursor-pointer hover:bg-indigo-600 text-white",
                        i { class: "bi bi-question-square px-2 py-1 rounded-md bg-slate-300 text-gray-900" },
                        span { class: "text-[15px], ml-4 text-gray-900 font-bold",
                            "Catch"
                        }
                    }
                }
            }
        }
        DiscoveredIp::Error(e) => {
            log::error!("Could not load poke to catch: {e}");
            rsx! {
                button {
                    class: "block w-full disabled",
                    div { class: "p-2.5 mt-3 flex items-center rounded-md px-4 duration-200 cursor-pointer hover:bg-indigo-600 text-white",
                        i { class: "bi bi-slash-circle px-2 py-1 rounded-md bg-slate-300 text-red-900" },
                        span { class: "text-[15px], ml-4 text-gray-900 font-bold",
                            "Something went wrong"
                        }
                    }
                }
            }
        }
        DiscoveredIp::Searching => {
            rsx! {
                button {
                    class: "block w-full readonly",
                    div { class: "p-2.5 mt-3 flex items-center rounded-md px-4 duration-200 cursor-pointer hover:bg-indigo-600 text-white",
                        i { class: "bi bi-circle-half px-2 py-1 rounded-md bg-slate-300 text-grey-900" },
                        span { class: "text-[15px], ml-4 text-gray-900 font-bold",
                            "Searching"
                        }
                    }
                }
            }
        }
        DiscoveredIp::Found => {
            rsx! {
                button {
                    class: "block w-full disabled",
                    div { class: "p-2.5 mt-3 flex items-center rounded-md px-4 duration-200 cursor-pointer hover:bg-indigo-600 text-white",
                        i { class: "bi bi-circle px-2 py-1 rounded-md bg-slate-300 text-grey-900" },
                        span { class: "text-[15px], ml-4 text-gray-900 font-bold",
                            "No pokemon"
                        }
                    }
                }
            }
        }
    };

    // TODO: support for exporting pokedex via QR code
    let toggle_qr = move |_evt| {};

    cx.render(rsx!(
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/bootstrap-icons@1.8.0/font/bootstrap-icons.css"},
        //aside { class: "sidebar fixed top-0 bottom-0 lg:left-0 p-2 w-[300px] overflow-y-auto text-center bg-slate-900",
        div { class: "h-screen flex",
            aside { class: "z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0",
                div { class: "h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800",
                    div { class: "text-gray-100 text-xl",
                        div { class: "p-2.5 mt-1 flex items-center",
                            i { class: "bi bi-app-indicator px-2 py-1 rounded-md bg-slate-300 text-gray-900" },
                            h1 { class: "font-bold text-gray-900 text-[15px] ml-3",
                                "PokeIpGo"
                            }
                        },
                        div { class: "my-2 bg-gray-600 h-[1px]"}
                    }
                    catch_button,
                    /*
                    button {
                        class: "block w-full",
                        onclick: update_random,
                        div { class: "p-2.5 mt-3 flex items-center rounded-md px-4 duration-200 cursor-pointer hover:bg-indigo-600 text-white",
                            i { class: "bi bi-question-square px-2 py-1 rounded-md bg-slate-300 text-gray-900" },
                            span { class: "text-[15px], ml-4 text-gray-900 font-bold",
                                "Catch"
                            }
                        }
                    }
                    */
                    button {
                        class: "block w-full",
                        onclick: update_random,
                        div { class: "p-2.5 mt-3 flex items-center rounded-md px-4 duration-200 cursor-pointer hover:bg-indigo-600 text-white",
                            i { class: "bi bi-plus-square px-2 py-1 rounded-md bg-slate-300 text-gray-900" },
                            span { class: "text-[15px], ml-4 text-gray-900 font-bold",
                                "Open Lootbox"
                            }
                        },
                    }
                    button {
                        class: "block w-full",
                        onclick: toggle_qr,
                        div { class: "p-2.5 mt-3 flex items-center rounded-md px-4 duration-200 cursor-pointer hover:bg-indigo-600 text-white",
                            i { class: "bi bi-qr-code-scan px-2 py-1 rounded-md bg-slate-300 text-gray-900" },
                            span { class: "text-[15px], ml-4 text-gray-900 font-bold",
                                "QR [TODO]"
                            }
                        },
                    }
                }
            }
            div { class: "flex-1 flex overflow-hidden",
                div { class: "flex-1 overflow-y-scroll",
                    section { class: "antialiased bg-gray-100 text-gray-600 px-4",
                    /*
                        div { class: "m-2 w-full text-center mx-auto",
                            catch_button,
                            button {
                                class: "inline w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                                onclick: update_random,
                                "Lootbox"
                            }
                            button {
                                class: "inline w-full md:w-auto p-6 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200 disabled",
                                onclick: toggle_qr,
                                "QR"
                            }
                        },
                        */
                        div { class: "",
                            div { class: "bg-white shadow-lg rounded-sm border border-gray-200",
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
                                                        div {class: "font-medium text-center",
                                                            "IP"
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
                                                                            sprite_url: p.sprite_url.clone()
                                                                        }
                                                                    }
                                                                    td { class: "p-2 whitespace-nowrap text-slate-500",
                                                                        div { class: "text-left",
                                                                            "{p.seed}"
                                                                        }
                                                                    }
                                                                    td { class: "p2 whitespace-nowrap text-slate-600",
                                                                        div { class: "text-left",
                                                                            "{ip_details.ip}"
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
                                                        MatchingPokemon::FailedToLoad(_fail) => {
                                                            rsx! {
                                                                span {
                                                                    "Failed" 
                                                                }
                                                            }
                                                        },
                                                        MatchingPokemon::Missing => {
                                                            rsx! {
                                                                span {
                                                                    "missingNo" 
                                                                }
                                                            }
                                                        },
                                                        MatchingPokemon::Loading(_id) => {
                                                            rsx! {
                                                                div { class: "flex justify-center items-center",
                                                                    div { class: "spinner-border animate-spin inline-block w-8 h-8 border-4 rounded-full",
                                                                        role: "status",
                                                                        span { class: "visually-hidden",
                                                                            "Loading..."
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                )}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}
