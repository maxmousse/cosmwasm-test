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

    #[test]
    fn new() {
        let name = "Project1".to_string();
        let creator_addr = Addr::unchecked("addr1");
        let project = Project::new(name.clone(), creator_addr.clone());

        assert_eq!(project.name, name);
        assert_eq!(project.creator_addr, creator_addr);
        assert_eq!(project.donations, vec![]);
    }
}
