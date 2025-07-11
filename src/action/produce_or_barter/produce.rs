pub mod recip;

use recip::{RecipBy, Search, dst::Dst, src::Src};

#[derive(Debug)]
pub struct Recip {
    pub src: Src,
    pub dst: Dst,
}

impl Recip {
    pub fn is_in(&self, book: &RecipBy<Src, Dst>) -> bool {
        book.search(&self.src).any(|dst| *dst == self.dst)
    }
}

impl From<(Src, Dst)> for Recip {
    fn from((src, dst): (Src, Dst)) -> Self {
        Self { src, dst }
    }
}
