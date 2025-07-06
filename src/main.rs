mod action;
mod card;
mod state;

use std::collections::BTreeMap;
use action::produce_or_barter::produce::{
    Recip,
    recip::{RecipBook, RecipBy, dst::Dst, src::Src},
};
use card::{product1::Product1, product2::Product2, resource::Resource};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use state::GameState;

#[allow(clippy::unwrap_used)]
fn main() -> anyhow::Result<()> {
    let seed = 1;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    println!("game is began.");
    let state = GameState::begin(&mut rng, 4)?;

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
    let state = state.try_produce_clone(&mut rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce clay.");
    let src = Src::default();
    let dst = Dst {
        dst: BTreeMap::from([(Resource::Clay.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(&mut rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    println!("produce compost.");
    let src = Src {
        src: BTreeMap::from([(Resource::Dung.into(), (1, 0).into()), (Resource::Clay.into(), (1, 0).into())]),
    };
    let dst = Dst {
        dst: BTreeMap::from([(Product1::Compost.into(), 1)]),
    };
    let recip = Recip { src, dst };
    let state = state.try_produce_clone(&mut rng, &recip, &book)?;
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
    let state = state.try_produce_clone(&mut rng, &recip, &book)?;
    println!("{:#?}", state.inventories.iter().peekable().next().unwrap());
    println!("{}", state.board);
    println!();

    Ok(())
}
