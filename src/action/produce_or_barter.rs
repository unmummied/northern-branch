pub mod barter;
pub mod produce;

use barter::Barter;
use produce::Recip;

pub type StockInt = u8;

#[derive(Debug)]
pub enum ProduceOrBarter<'a, T> {
    Produce { recip: Recip, book: &'a T },
    Barter(Barter),
}

impl<'a, T: 'a> From<(Recip, &'a T)> for ProduceOrBarter<'a, T> {
    fn from((recip, book): (Recip, &'a T)) -> Self {
        Self::Produce { recip, book }
    }
}
impl<'a, T: 'a> From<Barter> for ProduceOrBarter<'a, T> {
    fn from(barter: Barter) -> Self {
        Self::Barter(barter)
    }
}
