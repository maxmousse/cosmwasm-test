use cosmwasm_std::Addr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Donation {
    pub donator_addr: Addr,
    pub amount: u128,
}

impl Donation {
    pub fn new(donator_addr: Addr, amount: u128) -> Self {
        Donation {
            donator_addr,
            amount,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cw_multi_test::IntoBech32;

    #[test]
    fn new() {
        let donator_addr = "addr1".into_bech32();
        let amount = 1000u128;
        let donation = Donation::new(donator_addr.clone(), amount);

        assert_eq!(donation.donator_addr, donator_addr);
        assert_eq!(donation.amount, amount);
    }
}
