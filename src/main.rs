mod action;
mod card;

use action::produce_or_barter::produce::recip::{RecipBook, RecipBy, Search};
use card::product1::Product1;

fn main() {
    let data = RecipBook::data();
    println!("{:#?}", &data);

    println!();
    let data_by_src = RecipBy::from(data);
    println!("{:#?}", &data_by_src);

    println!();
    println!("Search by chicken");
    data_by_src
        .search(&[(Product1::Chicken, (1, 0))].into())
        .for_each(|dst| println!("{dst:#?}"));
}
