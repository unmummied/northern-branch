pub mod brownie;
pub mod produce_or_barter;

use brownie::Brownie;
use produce_or_barter::ProduceOrBarter;

pub struct Action<'a, T> {
    pub produce_or_barter: ProduceOrBarter<'a, T>,
    pub brownie: Brownie,
}
