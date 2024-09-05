use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use serde_json::Value;

use crate::models::{Config, NameMetadata};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct RegisterMsg {
    pub name: String,
    pub address: Addr,
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
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);
