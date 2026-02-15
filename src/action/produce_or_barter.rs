pub mod barter;
pub mod produce;

use barter::Barter;
use produce::Recipe;

pub type StockInt = u8;

#[derive(Debug)]
pub enum ProduceOrBarter<'a, T> {
    Produce { recipe: Recipe, book: &'a T },
    Barter(Barter),
}

impl<'a, T: 'a> From<(Recipe, &'a T)> for ProduceOrBarter<'a, T> {
    fn from((recipe, book): (Recipe, &'a T)) -> Self {
        Self::Produce { recipe, book }
    }
}
impl<'a, T: 'a> From<Barter> for ProduceOrBarter<'a, T> {
    fn from(barter: Barter) -> Self {
        Self::Barter(barter)
    }
}
