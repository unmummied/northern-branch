pub mod recipe;

use recipe::{RecipeBy, Search, dst::Dst, src::Src};

#[derive(Debug)]
pub struct Recipe {
    pub src: Src,
    pub dst: Dst,
}

impl Recipe {
    pub fn is_in(&self, book: &RecipeBy<Src, Dst>) -> bool {
        book.search(&self.src).any(|dst| *dst == self.dst)
    }
}

impl From<(Src, Dst)> for Recipe {
    fn from((src, dst): (Src, Dst)) -> Self {
        Self { src, dst }
    }
}
