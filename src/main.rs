mod action;
mod card;
mod state;

use action::produce_or_barter::{
    barter::Barter,
    produce::{
        Recip,
        recip::{RecipBook, RecipBy, dst::Dst, src::Src},
    },
};
use card::{
    building::basic::BasicBuilding, product1::Product1, product2::Product2, resource::Resource,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use state::GameState;
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    let seed = 1;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    test_case(&mut rng)
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::too_many_lines)]
fn test_case<R: Rng>(rng: &mut R) -> anyhow::Result<()> {
    println!("game is began.");
    let state = GameState::begin(rng, 4)?;

    let book = Into::<RecipBy<Src, Dst>>::into(RecipBook::data());
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce dung.");
    let src = Src::default();
    let dst = Dst {
        dst: BTreeMap::from([(Resource::Dung.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce clay.");
    let src = Src::default();
    let dst = Dst {
        dst: BTreeMap::from([(Resource::Clay.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce compost.");
    let src = Src {
        src: BTreeMap::from([
            (Resource::Dung.into(), (1, 0).into()),
            (Resource::Clay.into(), (1, 0).into()),
        ]),
    };
    let dst = Dst {
        dst: BTreeMap::from([(Product1::Compost.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce vegetables.");
    let src = Src {
        src: BTreeMap::from([(Product1::Compost.into(), (1, 0).into())]),
    };
    let dst = Dst {
        dst: BTreeMap::from([(Product2::Vegetables.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("give vegetable, take clay and 2 ores.");
    let give = Product2::Vegetables.into();
    let take = BTreeMap::from([(Resource::Clay.into(), 1), (Resource::Ore.into(), 2)]);
    let barter = Barter::Give1TakeN { give, take };
    let state = state.try_barter_clone(rng, &barter)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce bronze.");
    let src = Src {
        src: BTreeMap::from([(Resource::Ore.into(), (1, 0).into())]),
    };
    let dst = Dst {
        dst: BTreeMap::from([(Product1::Bronze.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce glass.");
    let src = Src {
        src: BTreeMap::from([
            (Resource::Ore.into(), (1, 0).into()),
            (Resource::Clay.into(), (1, 0).into()),
        ]),
    };
    let dst = Dst {
        dst: BTreeMap::from([(Product1::Glass.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce mirror.");
    let src = Src {
        src: BTreeMap::from([
            (Product1::Bronze.into(), (1, 0).into()),
            (Product1::Glass.into(), (1, 0).into()),
        ]),
    };
    let dst = Dst {
        dst: BTreeMap::from([(Product2::Mirror.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("give mirror, take smelter and glass factory.");
    let give = Product2::Mirror.into();
    let take = BTreeMap::from([
        (BasicBuilding::Smelter.into(), 1),
        (BasicBuilding::GlassFactory.into(), 1),
    ]);
    let barter = Barter::Give1TakeN { give, take };
    let state = state.try_barter_clone(rng, &barter)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    Ok(())
}
