use crate::ip::{Ip, IpSource};
use crate::poke_api;
use dioxus::prelude::*;
use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, WeightedIndex},
    SeedableRng,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonId(u32);

impl PokemonId {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn rustemon_id(&self) -> i64 {
        self.0 as i64
    }
}

impl From<PokemonSeed> for PokemonId {
    fn from(value: PokemonSeed) -> Self {
        let pokeseed = value.seed();
        let mut rng = rand::rngs::SmallRng::seed_from_u64(pokeseed);
        let index = POKE_WEIGHTED_INDEX.sample(&mut rng);
        let pid: PokemonId = POKE_DROP_RATES[index].0;

        log::info!("rolled pokemon: {pid:?}");
        pid
    }
}

impl From<(Ip, IpSource)> for PokemonSeed {
    fn from(value: (Ip, IpSource)) -> Self {
        let ip_bytes = u32::from(value.0);
        log::debug!("Ip: {ip_bytes:#?}");
        let pokeseed = SEED ^ ip_bytes as u64;
        log::debug!("seed: {pokeseed:#?}");

        match value.1 {
            IpSource::Network => PokemonSeed::Ip(pokeseed),
            IpSource::Lootbox => PokemonSeed::Lootbox(pokeseed),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
//pub struct PokemonSeed(u64);
pub enum PokemonSeed {
    Ip(u64),
    Lootbox(u64),
}

impl fmt::Display for PokemonSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (prefix, seed) = match self {
            PokemonSeed::Ip(v) => ("N", v),
            PokemonSeed::Lootbox(v) => ("L", v),
        };
        write!(f, "{prefix}{seed:010}")
    }
}

impl PokemonSeed {
    pub fn seed(&self) -> u64 {
        match self {
            PokemonSeed::Ip(v) => v,
            PokemonSeed::Lootbox(v) => v,
        }
        .clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: PokemonId,
    pub name: String,
    pub sprite_url: Option<String>,
    pub seed: PokemonSeed,
}

impl Pokemon {
    pub async fn new(seed: PokemonSeed) -> anyhow::Result<Self> {
        fn get_available_sprite(sprites: poke_api::PokemonSprites) -> Option<String> {
            let poke_api::PokemonSprites {
                front_default,
                back_default,
            } = sprites;
            front_default.or(back_default)
        }
        let id = PokemonId::from(seed);

        let pokemon = poke_api::get_by_id(id.rustemon_id()).await?;

        Ok(Self {
            id,
            name: pokemon.name,
            sprite_url: get_available_sprite(pokemon.sprites),
            seed,
        })
    }
}

pub mod draw {
    use super::*;

    #[derive(Props, PartialEq)]
    pub struct PokemonProps {
        //id: PokemonId,
        name: String,
        //seed: String,
        #[props(!optional)]
        sprite_url: Option<String>,
    }

    #[allow(non_snake_case)]
    pub fn Pokemon(cx: Scope<PokemonProps>) -> Element {
        let sprite = match &cx.props.sprite_url {
            Some(url) => {
                rsx! {
                    div { class: "w-10 h-10 flex-shring-0 mr-2 sm:mr-3",
                        img { class: "rounded-full",
                            src: "{url}",
                            width: "80",
                            height: "80",
                            alt: "{cx.props.name} sprite",
                        }
                    }
                }
            }
            None => rsx!("unimplemented"),
        };

        let span = cx.render(rsx! {
            div { class: "flex items-center",
                sprite
                div { class: "font-medium text-gray-800",
                    "{cx.props.name}"
                }
            }
        });

        span
    }
}

lazy_static! {
    static ref POKE_WEIGHTED_INDEX: WeightedIndex<u16> =
        WeightedIndex::new(POKE_DROP_RATES.iter().map(|item| item.1)).unwrap();
}

/// we re-init rng with the same seed
const SEED: u64 = 1365493862;

const POKE_DROP_RATES: &[(PokemonId, u16)] = &[
    (PokemonId(1), 45),
    (PokemonId(2), 45),
    (PokemonId(3), 45),
    (PokemonId(4), 45),
    (PokemonId(5), 45),
    (PokemonId(6), 45),
    (PokemonId(7), 45),
    (PokemonId(8), 45),
    (PokemonId(9), 45),
    (PokemonId(10), 255),
    (PokemonId(11), 120),
    (PokemonId(12), 45),
    (PokemonId(13), 255),
    (PokemonId(14), 120),
    (PokemonId(15), 45),
    (PokemonId(16), 255),
    (PokemonId(17), 120),
    (PokemonId(18), 45),
    (PokemonId(19), 255),
    (PokemonId(20), 127),
    (PokemonId(21), 255),
    (PokemonId(22), 90),
    (PokemonId(23), 255),
    (PokemonId(24), 90),
    (PokemonId(25), 190),
    (PokemonId(26), 75),
    (PokemonId(27), 255),
    (PokemonId(28), 90),
    (PokemonId(29), 235),
    (PokemonId(30), 120),
    (PokemonId(31), 45),
    (PokemonId(32), 235),
    (PokemonId(33), 120),
    (PokemonId(34), 45),
    (PokemonId(35), 150),
    (PokemonId(36), 25),
    (PokemonId(37), 190),
    (PokemonId(38), 75),
    (PokemonId(39), 170),
    (PokemonId(40), 50),
    (PokemonId(41), 255),
    (PokemonId(42), 90),
    (PokemonId(43), 255),
    (PokemonId(44), 120),
    (PokemonId(45), 45),
    (PokemonId(46), 190),
    (PokemonId(47), 75),
    (PokemonId(48), 190),
    (PokemonId(49), 75),
    (PokemonId(50), 255),
    (PokemonId(51), 50),
    (PokemonId(52), 255),
    (PokemonId(53), 90),
    (PokemonId(54), 190),
    (PokemonId(55), 75),
    (PokemonId(56), 190),
    (PokemonId(57), 75),
    (PokemonId(58), 190),
    (PokemonId(59), 75),
    (PokemonId(60), 255),
    (PokemonId(61), 120),
    (PokemonId(62), 45),
    (PokemonId(63), 200),
    (PokemonId(64), 100),
    (PokemonId(65), 50),
    (PokemonId(66), 180),
    (PokemonId(67), 90),
    (PokemonId(68), 45),
    (PokemonId(69), 255),
    (PokemonId(70), 120),
    (PokemonId(71), 45),
    (PokemonId(72), 190),
    (PokemonId(73), 60),
    (PokemonId(74), 255),
    (PokemonId(75), 120),
    (PokemonId(76), 45),
    (PokemonId(77), 190),
    (PokemonId(78), 60),
    (PokemonId(79), 190),
    (PokemonId(80), 75),
    (PokemonId(81), 190),
    (PokemonId(82), 60),
    (PokemonId(83), 45),
    (PokemonId(84), 190),
    (PokemonId(85), 45),
    (PokemonId(86), 190),
    (PokemonId(87), 75),
    (PokemonId(88), 190),
    (PokemonId(89), 75),
    (PokemonId(90), 190),
    (PokemonId(91), 60),
    (PokemonId(92), 190),
    (PokemonId(93), 90),
    (PokemonId(94), 45),
    (PokemonId(95), 45),
    (PokemonId(96), 190),
    (PokemonId(97), 75),
    (PokemonId(98), 225),
    (PokemonId(99), 60),
    (PokemonId(100), 190),
    (PokemonId(101), 60),
    (PokemonId(102), 90),
    (PokemonId(103), 45),
    (PokemonId(104), 190),
    (PokemonId(105), 75),
    (PokemonId(106), 45),
    (PokemonId(107), 45),
    (PokemonId(108), 45),
    (PokemonId(109), 190),
    (PokemonId(110), 60),
    (PokemonId(111), 120),
    (PokemonId(112), 60),
    (PokemonId(113), 30),
    (PokemonId(114), 45),
    (PokemonId(115), 45),
    (PokemonId(116), 225),
    (PokemonId(117), 75),
    (PokemonId(118), 225),
    (PokemonId(119), 60),
    (PokemonId(120), 225),
    (PokemonId(121), 60),
    (PokemonId(122), 45),
    (PokemonId(123), 45),
    (PokemonId(124), 45),
    (PokemonId(125), 45),
    (PokemonId(126), 45),
    (PokemonId(127), 45),
    (PokemonId(128), 45),
    (PokemonId(129), 255),
    (PokemonId(130), 45),
    (PokemonId(131), 45),
    (PokemonId(132), 35),
    (PokemonId(133), 45),
    (PokemonId(134), 45),
    (PokemonId(135), 45),
    (PokemonId(136), 45),
    (PokemonId(137), 45),
    (PokemonId(138), 45),
    (PokemonId(139), 45),
    (PokemonId(140), 45),
    (PokemonId(141), 45),
    (PokemonId(142), 45),
    (PokemonId(143), 25),
    (PokemonId(144), 3),
    (PokemonId(145), 3),
    (PokemonId(146), 3),
    (PokemonId(147), 45),
    (PokemonId(148), 45),
    (PokemonId(149), 45),
    (PokemonId(150), 3),
    (PokemonId(151), 45),
];
