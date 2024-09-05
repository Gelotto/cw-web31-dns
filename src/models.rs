use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
    pub contract: Addr,
    pub created_at: Timestamp,
}

#[cw_serde]
pub struct NameMetadata {
    pub title: Option<String>,
    pub desription: Option<String>,
    pub keywords: Vec<String>,
}

#[cw_serde]
pub struct PublicNameRecord {
    pub owner: Addr,
    pub cannonical_name: String,
    pub contract: Addr,
    pub created_at: Timestamp,
    pub meta: NameMetadata,
}
