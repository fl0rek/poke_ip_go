use crate::ip::Ip;
use dioxus::prelude::*;
use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, WeightedIndex},
    SeedableRng,
};
use rustemon::{client::RustemonClient, model::pokemon::PokemonSprites};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PokemonId(u32);

impl PokemonId {
    pub fn id(&self) -> u32 {
        self.0
    }

    pub fn rustemon_id(&self) -> i64 {
        self.0 as i64
    }
}

pub struct Pokemon {
    //id: i64,
    pub name: String,
    pub sprite_url: Option<String>,
}

impl Pokemon {
    pub async fn new(client: &RustemonClient, id: PokemonId) -> anyhow::Result<Self> {
        fn get_available_sprite(sprites: PokemonSprites) -> Option<String> {
            let PokemonSprites {
                front_default,
                back_default,
                ..
            } = sprites;
            front_default.or(back_default)
        }

        let pokemon = rustemon::pokemon::pokemon::get_by_id(id.rustemon_id(), client).await?;

        Ok(Self {
            //id,
            name: pokemon.name,
            sprite_url: get_available_sprite(pokemon.sprites),
        })
    }
}

pub mod draw {
    use super::*;

    #[derive(Props, PartialEq)]
    pub struct PokemonProps {
        //id: PokemonId,
        name: String,

        #[props(!optional)]
        sprite_url: Option<String>,
    }

    pub fn Pokemon(cx: Scope<PokemonProps>) -> Element {
        //println!("draw start");
        //let pokemon = use_state(&cx, || None);
        //let id = cx.props.id;

        /*
        cx.spawn({
            to_owned![pokemon];
            async move {
            pokemon.set(Some(super::Pokemon::new(&RUSTEMON_CLIENT, id).await
                .map_err(|e| unimplemented!("Could not get pokemon: {e}")).unwrap()));
        }});
        */

        let sprite = match &cx.props.sprite_url {
            Some(url) => {
                rsx! {
                    img {
                        src: "{url}",
                        alt: "sprite"
                    }
                }
            }
            None => rsx!("unimplemented"),
        };

        let a = cx.render(rsx! {
            span {
                "{cx.props.name}"
            }
            sprite
        });
        //println!("draw end");
        a
    }
}

impl From<Ip> for PokemonId {
    fn from(value: Ip) -> Self {
        let ip_bytes = u32::from(value);
        println!("Ip: {ip_bytes:#?}");
        let pokeseed = SEED ^ ip_bytes as u64;
        println!("seed: {pokeseed:#?}");
        //let rng: SeedableRng<Seed = u64> = SeedableRng::seed_from_u64(pokeseed);
        let mut chacha = rand_chacha::ChaCha8Rng::seed_from_u64(pokeseed);
        println!("chacha ok");
        let index = POKE_WEIGHTED_INDEX.sample(&mut chacha);
        println!("index: {index:#?}");
        let r = POKE_DROP_RATES[index].0;

        println!("{r:#?}");

        r
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
