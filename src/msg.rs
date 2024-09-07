use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use serde_json::Value;

use crate::{
    models::{Config, NameMetadata, PublicNameRecord},
    token::TokenAmount,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub price: TokenAmount,
    pub fee_recipient: Addr,
    pub max_name_len: u8,
}

#[cw_serde]
pub struct RegisterMsg {
    pub owner: Addr,
    pub name: String,
    pub address: String,
    pub meta: Option<NameMetadata>,
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    Register(RegisterMsg),
}

#[cw_serde]
pub struct RenderQueryMsg {
    pub contract: String,
    pub path: String,
    pub context: Option<Value>,
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(String)]
    Render(RenderQueryMsg),

    #[returns(PublicNameRecord)]
    NameRecord { contract: String },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);
