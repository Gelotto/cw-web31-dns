use cosmwasm_std::{Addr, Deps, Response};
use cw_storage_plus::{Item, Map};

use crate::{
    error::ContractError,
    execute::Context,
    models::{NameMetadata, NameRecord},
    msg::InstantiateMsg,
    token::TokenAmount,
    utils::is_bech32_address,
};

pub const PRICE: Item<TokenAmount> = Item::new("unit_price");
pub const FEE_RECIPIENT: Item<Addr> = Item::new("fee_recipient");
pub const MAX_NAME_LEN: Item<u8> = Item::new("max_name_len");
pub const NAME_RECORDS: Map<&String, NameRecord> = Map::new("name_records");
pub const CONTRACT_ADDR_2_NAME: Map<&String, String> = Map::new("contract_addr_2_name");
pub const NAME_METADATA: Map<&String, NameMetadata> = Map::new("name_metadata");

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    PRICE.save(deps.storage, &msg.price)?;
    FEE_RECIPIENT.save(deps.storage, &deps.api.addr_validate(&msg.fee_recipient.as_str())?)?;
    MAX_NAME_LEN.save(deps.storage, &msg.max_name_len.max(1))?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub fn resolve_contract_address(
    deps: &Deps,
    addr_or_name: &String,
) -> Result<String, ContractError> {
    if is_bech32_address(addr_or_name) {
        Ok(addr_or_name.to_owned())
    } else if let Some(NameRecord { contract, .. }) = NAME_RECORDS.may_load(deps.storage, addr_or_name)? {
        Ok(contract)
    } else {
        Err(ContractError::NotFound {
            reason: format!("could not resolve contract address from {}", addr_or_name),
        })
    }
}
