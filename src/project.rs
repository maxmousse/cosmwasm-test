use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

use crate::donation::Donation;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub creator_addr: Addr,
    pub donations: Vec<Donation>,
}

impl Project {
    pub fn new(name: String, creator_addr: Addr) -> Self {
        Project {
            name,
            creator_addr,
            donations: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cw_multi_test::IntoBech32;

    #[test]
    fn new() {
        let name = "Project1".to_string();
        let creator_addr = "addr1".into_bech32();
        let project = Project::new(name.clone(), creator_addr.clone());

        assert_eq!(project.name, name);
        assert_eq!(project.creator_addr, creator_addr);
        assert_eq!(project.donations, vec![]);

        //TODO: remove all the text below this line, just placed here to show some address generation usages
        assert_eq!(
            "addr1".into_bech32().as_str(),
            "cosmwasm14ch5q26mhx3jk5cxl88t278nper264ce5fa7agjr4cw0yfjj7c6q56drym",
        );
        assert_eq!(
            "addr1".into_bech32_with_prefix("juno").as_str(),
            "juno14ch5q26mhx3jk5cxl88t278nper264ce5fa7agjr4cw0yfjj7c6q96gycm",
        );
    }
}
