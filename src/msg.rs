use serde::{Deserialize, Serialize};

use crate::donation::Donation;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub donation_denom: String,
    pub fee_collector_addr: String,
    pub projects: Vec<(String, String)>, // (name, creator_addr)
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    GetDonationsByDonator { donator_addr: String },
    GetDonationsByProject { project_creator_addr: String },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    Donate { project_creator_addr: String },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GetDonationsResponse {
    pub donations: Vec<Donation>,
}
