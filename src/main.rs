mod action;
mod card;
mod state;

use anyhow::Error;
use rand::rng;
use state::board::BoardState;

fn main() -> anyhow::Result<()> {
    let mut binding = BoardState::new_n(4).map_err(Error::msg)?;

    let mut rng = rng();
    binding.fill_slots(&mut rng);

    println!("{binding}");

    Ok(())
}
