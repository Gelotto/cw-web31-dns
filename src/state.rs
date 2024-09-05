use cosmwasm_std::{Addr, Deps, Response};
use cw_storage_plus::{Item, Map};

use crate::{
    error::ContractError,
    execute::Context,
    models::{NameMetadata, NameRecord},
    msg::InstantiateMsg,
    token::TokenAmount,
};

pub const UNIT_PRICE: Item<TokenAmount> = Item::new("unit_price");
pub const FEE_RECIPIENT: Item<Addr> = Item::new("fee_recipient");
pub const NAME_RECORDS: Map<&String, NameRecord> = Map::new("name_records");
pub const CONTRACT_ADDR_2_NAME: Map<&Addr, String> = Map::new("contract_addr_2_name");
pub const NAME_METADATA: Map<&String, NameMetadata> = Map::new("name_metadata");

/// Top-level initialization of contract state
pub fn init(
    _ctx: Context,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub fn resolve_contract_address(
    deps: &Deps,
    addr_or_name: &String,
) -> Result<Addr, ContractError> {
    if let Ok(contract_addr) = deps.api.addr_validate(addr_or_name) {
        Ok(contract_addr)
    } else if let Some(NameRecord { contract, .. }) = NAME_RECORDS.may_load(deps.storage, addr_or_name)? {
        Ok(contract)
    } else {
        Err(ContractError::NotFound {
            reason: format!("could not resolve contract address"),
        })
    }
}
