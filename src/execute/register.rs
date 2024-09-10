use crate::{
    error::ContractError,
    models::NameRecord,
    msg::RegisterMsg,
    state::{FEE_RECIPIENT, NAME_METADATA, NAME_RECORDS, PRICE},
    token::TokenAmount,
    utils::is_bech32_address,
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_register(
    ctx: Context,
    msg: RegisterMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;

    let RegisterMsg {
        owner,
        name,
        address: contract_addr,
        meta,
    } = msg;

    let TokenAmount { token, amount: price } = PRICE.load(deps.storage)?;

    let cannonical_name = name.to_ascii_lowercase();

    // Ensure user has sent payment
    if token.find_in_funds(&info.funds, Some(price)).is_none() {
        return Err(ContractError::InsufficientFunds {
            exp_amount: price.into(),
        });
    }

    // Ensure the address string is a valid bech32 address
    if !is_bech32_address(&contract_addr) {
        return Err(ContractError::ValidationError {
            reason: format!("{} is not a valid bech32 address", &contract_addr),
        });
    }

    // Add transfer submsg to response to send platform fee
    let resp = Response::new().add_submessage(token.transfer(&FEE_RECIPIENT.load(deps.storage)?, price)?);

    // Create a name record or error out if already exists
    NAME_RECORDS.update(
        deps.storage,
        &cannonical_name,
        |maybe_record| -> Result<_, ContractError> {
            if maybe_record.is_some() {
                return Err(ContractError::NameExists { name });
            }
            Ok(NameRecord {
                contract: contract_addr.to_owned(),
                created_at: env.block.time,
                owner: deps.api.addr_validate(&owner.as_str())?,
            })
        },
    )?;

    // Save or init empty metadata for the NameRecord
    NAME_METADATA.save(
        deps.storage,
        &cannonical_name,
        &meta.unwrap_or_default(),
    )?;

    Ok(resp.add_attributes(vec![
        attr("action", "register"),
        attr("name", cannonical_name),
        attr("contract", contract_addr),
        attr("owner", owner.to_string()),
    ]))
}
