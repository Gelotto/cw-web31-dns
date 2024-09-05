use crate::{
    error::ContractError,
    models::{NameMetadata, NameRecord},
    msg::RegisterMsg,
    state::{FEE_RECIPIENT, NAME_METADATA, NAME_RECORDS, PRICE},
    token::TokenAmount,
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_register(
    ctx: Context,
    msg: RegisterMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;
    let RegisterMsg { name, address, meta } = msg;
    let TokenAmount { token, amount: price } = PRICE.load(deps.storage)?;

    // Ensure user has sent payment
    if token.find_in_funds(&info.funds, Some(price)).is_none() {
        return Err(ContractError::InsufficientFunds {
            exp_amount: price.into(),
        });
    }

    // Add transfer submsg to response to send platform fee
    let resp = Response::new().add_submessage(token.transfer(&FEE_RECIPIENT.load(deps.storage)?, price)?);

    // Create a name record or error out if already exists
    let contract_addr = deps.api.addr_validate(address.as_str())?;
    let cannonical_name = name.to_ascii_lowercase();

    NAME_RECORDS.update(
        deps.storage,
        &cannonical_name,
        |maybe_record| -> Result<_, ContractError> {
            if maybe_record.is_some() {
                return Err(ContractError::NameExists { name });
            }
            Ok(NameRecord {
                contract: contract_addr,
                created_at: env.block.time,
                owner: info.sender.to_owned(),
            })
        },
    )?;

    // Save or init empty metadata for the NameRecord
    NAME_METADATA.save(
        deps.storage,
        &cannonical_name,
        &meta.unwrap_or_else(|| NameMetadata {
            title: None,
            desription: None,
            keywords: vec![],
        }),
    )?;

    Ok(resp.add_attributes(vec![
        attr("action", "register"),
        attr("name", cannonical_name),
        attr("address", address.to_string()),
    ]))
}
