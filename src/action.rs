pub mod produce_or_barter;

use produce_or_barter::ProduceOrBarter;

pub struct Action<'a, T> {
    pub produce_or_barter: ProduceOrBarter<'a, T>,
    // pub brownie: Brownie,
}
