use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::project::Project;

pub const FEE_COLLECTOR_ADDR: Item<Addr> = Item::new("fee_collector_addr");
pub const PROJECTS: Item<Vec<Project>> = Item::new("projects");
