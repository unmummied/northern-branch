mod action;
mod card;
mod state;

use anyhow::Error;
use rand::rng;
use state::board::BoardState;

fn main() -> anyhow::Result<()> {
    let mut rng = rng();
    let mut binding = BoardState::deal(&mut rng, 2).map_err(Error::msg)?;

    binding.fill_slots(&mut rng);
    println!("{binding}");

    Ok(())
}
