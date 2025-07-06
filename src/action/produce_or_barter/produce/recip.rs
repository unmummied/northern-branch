pub mod dst;
pub mod src;

use crate::card::{
    Card,
    product1::Product1::{Bronze, Cement, Chicken, Compost, Fuel, Glass, Mushroom, Pig, Timber},
    product2::Product2::{
        Beer, Broadax, Concrete, Down, Egg, Furniture, Mirror, Sausage, Vegetables,
    },
    resource::Resource::{Barley, Clay, Dung, Ore, Wood},
};
use dst::Dst;
use src::Src;
use std::{
    collections::{BTreeMap, BTreeSet},
    iter,
};

pub trait Search<'a> {
    type Key;
    type Val: 'a;
    fn search(&'a self, key: &Self::Key) -> impl Iterator<Item = &'a Self::Val>;
}

type BookIndexInt = u8;
#[derive(Debug, Default)]
pub struct RecipBy<K, V> {
    pub recips: BTreeMap<(K, BookIndexInt), V>,
}

impl<'a, K: Ord + Clone, V: 'a> Search<'a> for RecipBy<K, V> {
    type Key = K;
    type Val = V;
    fn search(&'a self, key: &K) -> impl Iterator<Item = &'a V> {
        self.recips
            .range((key.clone(), BookIndexInt::MIN)..)
            .take_while(move |((k, _), _)| k == key)
            .map(|(_, v)| v)
    }
}

impl From<RecipBook> for RecipBy<Src, Dst> {
    fn from(book: RecipBook) -> Self {
        let mut grouped = BTreeMap::<_, Vec<_>>::new();
        for (src, dst) in book.recips {
            grouped.entry(src).or_default().push(dst);
        }

        let mut recips = BTreeMap::new();
        for (src, mut dsts) in grouped {
            dsts.sort();
            #[allow(clippy::cast_possible_truncation)]
            for (i, dst) in dsts.into_iter().enumerate() {
                recips.insert((src.clone(), i as _), dst);
            }
        }
        Self { recips }
    }
}

#[derive(Debug, Default)]
pub struct RecipBook {
    pub recips: BTreeSet<(Src, Dst)>,
}

impl RecipBook {
    fn extend(&mut self, rhs: Self) {
        self.recips.extend(rhs.recips);
    }
}

impl<S, D, I> From<I> for RecipBook
where
    I: IntoIterator<Item = (S, D)>,
    D: Into<Dst>,
    S: Into<Src>,
{
    fn from(iterable: I) -> Self {
        Self {
            recips: iterable
                .into_iter()
                .map(|(src, dst)| (src.into(), dst.into()))
                .collect(),
        }
    }
}

impl RecipBook {
    /// Warning: this associated function is too big.
    pub fn data() -> Self {
        let free = [
            [(Src::default(), [(Dung, 1)])].into(),
            [(Src::default(), [(Clay, 1)])].into(),
            [(Src::default(), [(Barley, 1)])].into(),
            [(Src::default(), [(Wood, 1)])].into(),
            [(Src::default(), [(Ore, 1)])].into(),
        ]
        .into_iter();

        let from_resources = [
            [([(Dung, (1, 0))], [(Fuel, 1)])].into(),
            [([(Clay, (1, 0))], [(Cement, 1)])].into(),
            [([(Barley, (1, 0))], [(Pig, 1)])].into(),
            [([(Wood, (1, 0))], [(Mushroom, 1)])].into(),
            [([(Wood, (1, 0))], [(Timber, 1)])].into(),
            [([(Ore, (1, 0))], [(Bronze, 1)])].into(),
            [([(Dung, (1, 0)), (Clay, (1, 0))], [(Compost, 1)])].into(),
            [([(Clay, (1, 0)), (Barley, (1, 0))], [(Chicken, 1)])].into(),
            [([(Clay, (1, 0)), (Ore, (1, 0))], [(Glass, 1)])].into(),
        ]
        .into_iter();

        let from_resource_and_product1 = [
            [(
                [(Card::from(Barley), (1, 0)), (Timber.into(), (1, 0))],
                [(Beer, 1)],
            )]
            .into(),
            [(
                [(Card::from(Wood), (1, 0)), (Bronze.into(), (1, 0))],
                [(Broadax, 1)],
            )]
            .into(),
        ]
        .into_iter();

        let from_product1 = [
            [([(Compost, (1, 0))], [(Vegetables, 1)])].into(),
            [([(Cement, (1, 0))], [(Concrete, 1)])].into(),
            [([(Chicken, (1, 0))], [(Down, 1)])].into(),
            [([(Chicken, (1, 0))], [(Sausage, 1)])].into(),
            [([(Chicken, (0, 1))], [(Egg, 1)])].into(),
            [([(Pig, (1, 0))], [(Sausage, 1)])].into(),
            [([(Pig, (0, 1))], [(Mushroom, 1)])].into(),
            [([(Timber, (1, 0))], [(Furniture, 1)])].into(),
        ]
        .into_iter();

        let from_product1s =
            std::iter::once([([(Bronze, (1, 0)), (Glass, (1, 0))], [(Mirror, 1)])].into());

        let from_product2 = std::iter::once([([(Broadax, (0, 1))], [(Timber, 1)])].into());

        iter::once(Self::default())
            .chain(free)
            .chain(from_resources)
            .chain(from_resource_and_product1)
            .chain(from_product1)
            .chain(from_product1s)
            .chain(from_product2)
            .fold(Self::default(), |mut acc, book| {
                acc.extend(book);
                acc
            })
    }
}
