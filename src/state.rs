pub mod board;
pub mod inventory;
pub mod queue;

use crate::action::produce_or_barter::produce::{
    Recip,
    recip::{RecipBy, dst::Dst, src::Src},
};
use anyhow::{Context, anyhow};
use board::BoardState;
use inventory::Inventory;
use queue::{Name, Queue};
use rand::Rng;
use std::collections::BTreeMap;

pub type PopulationInt = usize;

const QUEUE_IS_BROKEN: &str = "`self.queue` is broken...";
const INVENTORY_IS_NOT_ENOUGH: &str = "inventory is not enough...";

#[derive(Debug, Clone)]
pub struct GameState {
    queue: Queue,
    pub inventories: BTreeMap<Name, Inventory>,
    pub board: BoardState,
}

impl GameState {
    pub fn begin<R: Rng>(rng: &mut R, population: PopulationInt) -> anyhow::Result<Self> {
        let queue = Queue::try_from(population).map_err(|e| anyhow!(e))?;
        let inventories = queue
            .members()
            .map(|name| (name, Inventory::default()))
            .collect();
        let board = BoardState::with_deal(rng, population)?;
        Ok(Self {
            queue,
            inventories,
            board,
        })
    }

    pub fn try_produce_clone<R: Rng>(
        &self,
        rng: &mut R,
        recip: &Recip,
        book: &RecipBy<Src, Dst>,
    ) -> anyhow::Result<Self> {
        let mut res = self.clone();
        let player = self
            .queue
            .peek()
            .context(QUEUE_IS_BROKEN)
            .map_err(|e| anyhow!(e))?;

        // update inventory
        let prev_inventory = self
            .inventories
            .get(&player)
            .context(INVENTORY_IS_NOT_ENOUGH)
            .map_err(|e| anyhow!(e))?;
        let next_inventory = prev_inventory
            .clone()
            .try_produce_clone(recip, book)
            .map_err(|e| anyhow!(e))?;
        res.inventories.insert(player, next_inventory);

        // update board state
        let next_board = self
            .board
            .try_produce_clone(rng, &recip.dst)
            .map_err(|e| anyhow!(e))?;
        res.board = next_board;
        res.board.discard_src(&recip.src);

        Ok(res)
    }
}
