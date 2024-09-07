use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
    pub contract: String,
    pub created_at: Timestamp,
}

#[cw_serde]
pub struct NameMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub favicon: Option<ImageAsset>,
    pub logo: Option<ImageAsset>,
    pub keywords: Vec<String>,
}

#[cw_serde]
pub enum ImageAsset {
    Svg(String),
    Url(String),
}

#[cw_serde]
pub struct PublicNameRecord {
    pub owner: Addr,
    pub cannonical_name: String,
    pub contract: String,
    pub created_at: Timestamp,
    pub meta: NameMetadata,
}
