use cosmwasm_schema::cw_serde;
use serde_json::Value;

use crate::{error::ContractError, msg::RenderQueryMsg, state::resolve_contract_address};

use super::ReadonlyContext;

#[cw_serde]
pub struct RenderParams {
    pub path: String,
    pub context: Option<Value>,
}

/// Lookup a contract by name or address and proxy pass the template path and
/// rendering context to it's own render smart query.
pub fn query_render(
    ctx: ReadonlyContext,
    msg: RenderQueryMsg,
) -> Result<String, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let RenderQueryMsg {
        contract,
        path,
        context,
    } = msg;

    // Get downstream contract address from given name or address
    let contract_addr = resolve_contract_address(&deps, &contract)?;

    // Render and return HTML
    Ok(deps
        .querier
        .query_wasm_smart(contract_addr, &RenderParams { path, context })?)
}
