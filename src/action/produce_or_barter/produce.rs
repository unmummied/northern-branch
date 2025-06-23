pub mod recip;

use recip::{RecipBy, Search, dst::Dst, src::Src};

#[derive(Debug)]
pub struct Recip {
    pub src: Src,
    pub dst: Dst,
}

impl Recip {
    pub fn is_valid(&self, book: &RecipBy<Src, Dst>) -> bool {
        book.search(&self.src).any(|dst| *dst == self.dst)
    }
}
