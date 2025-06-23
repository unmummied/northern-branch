pub mod barter;
pub mod produce;

use barter::Barter;
use produce::Recip;

type RecipInt = u8;

#[derive(Debug)]
pub enum ProduceOrBarter<'a, T> {
    Produce { recip: Recip, book: &'a T },
    Barter(Barter),
}
