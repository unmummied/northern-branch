pub mod board;
pub mod inventory;
pub mod queue;

use crate::action::produce_or_barter::{
    ProduceOrBarter,
    barter::Barter,
    produce::{
        Recipe,
        recipe::{RecipeBy, dst::Dst, src::Src},
    },
};
use anyhow::{Context, anyhow};
use board::BoardState;
use inventory::{ERR_FAILED_FORCE_INTO_GIVE_N_TAKE_N, Inventory};
use queue::{Name, Queue};
use rand::Rng;
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

pub type PopulationInt = usize;

const ERR_QUEUE_IS_BROKEN: &str = "`self.queue` is broken...";
const ERR_INVENTORY_IS_NOT_ENOUGH: &str = "inventory is not enough...";
const ERR_PEEK_IS_FAILED: &str = "peeking is failed...";

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

    pub fn curr_player_inventory(&self) -> Option<&Inventory> {
        if let Some(name) = self.queue.curr_player() {
            return self.inventories.get(&name);
        }
        None
    }

    fn try_produce_clone<R: Rng>(
        &self,
        rng: &mut R,
        recipe: &Recipe,
        book: &RecipeBy<Src, Dst>,
    ) -> anyhow::Result<Self> {
        let mut res = self.clone();
        let player = self
            .queue
            .curr_player()
            .context(ERR_QUEUE_IS_BROKEN)
            .map_err(|e| anyhow!(e))?;

        // update inventory
        let prev_inventory = self
            .inventories
            .get(&player)
            .context(ERR_INVENTORY_IS_NOT_ENOUGH)
            .map_err(|e| anyhow!(e))?;
        let next_inventory = prev_inventory
            .clone()
            .try_produce_clone(recipe, book)
            .map_err(|e| anyhow!(e))?;
        res.inventories.insert(player, next_inventory);

        // update board state
        let next_board = self
            .board
            .try_produce_clone(rng, &recipe.dst)
            .map_err(|e| anyhow!(e))?;
        res.board = next_board;
        res.board.discard_src(&recipe.src);

        Ok(res)
    }

    fn try_barter_clone<R: Rng>(&self, rng: &mut R, barter: &Barter) -> anyhow::Result<Self> {
        let mut res = self.clone();
        let player = self
            .queue
            .curr_player()
            .context(ERR_QUEUE_IS_BROKEN)
            .map_err(|e| anyhow!(e))?;

        // update inventory
        let prev_inventory = self
            .inventories
            .get(&player)
            .context(ERR_INVENTORY_IS_NOT_ENOUGH)
            .map_err(|e| anyhow!(e))?;
        let next_inventory = prev_inventory
            .clone()
            .try_barter_clone(barter)
            .map_err(|e| anyhow!(e))?;
        res.inventories.insert(player, next_inventory);

        // update board state
        let Barter::GiveNTakeN { give, take } = barter.clone().force_into_give_n_take_n() else {
            return Err(anyhow!(ERR_FAILED_FORCE_INTO_GIVE_N_TAKE_N));
        };
        let next_board = self
            .board
            .try_barter_clone(rng, &take)
            .map_err(|e| anyhow!(e))?;
        res.board = next_board;
        res.board.discard_given(&give);

        Ok(res)
    }

    pub fn try_produce_or_barter_clone<R: Rng>(
        &self,
        rng: &mut R,
        produce_or_barter: &ProduceOrBarter<RecipeBy<Src, Dst>>,
    ) -> anyhow::Result<Self> {
        match produce_or_barter {
            ProduceOrBarter::Produce { recipe, book } => self.try_produce_clone(rng, recipe, book),
            ProduceOrBarter::Barter(barter) => self.try_barter_clone(rng, barter),
        }
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let curr_player = self
            .queue
            .curr_player()
            .context(ERR_QUEUE_IS_BROKEN)
            .map_err(|_| fmt::Error)?;
        writeln!(f, "Queue: {}", &self.queue)?;
        writeln!(
            f,
            "{curr_player:?} has {}",
            self.inventories
                .get(&curr_player)
                .context(ERR_PEEK_IS_FAILED)
                .map_err(|_| fmt::Error)?
        )?;
        write!(f, "{}", self.board)?;
        Ok(())
    }
}
