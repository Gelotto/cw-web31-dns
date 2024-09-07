use crate::{
    error::ContractError,
    models::PublicNameRecord,
    state::{CONTRACT_ADDR_2_NAME, NAME_METADATA, NAME_RECORDS},
    utils::is_bech32_address,
};

use super::ReadonlyContext;

/// Return a NameRecord by name
pub fn query_name_record(
    ctx: ReadonlyContext,
    contract: String,
) -> Result<Option<PublicNameRecord>, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    // Find NameRecord by contract address or name
    if let Some(cannonical_name) = if is_bech32_address(&contract) {
        CONTRACT_ADDR_2_NAME.may_load(deps.storage, &contract)?
    } else {
        Some(contract.to_ascii_lowercase())
    } {
        if let Some(record) = NAME_RECORDS.may_load(deps.storage, &cannonical_name)? {
            // Build and return public NameRecord
            let meta = NAME_METADATA.load(deps.storage, &cannonical_name)?;
            return Ok(Some(PublicNameRecord {
                owner: record.owner,
                contract: record.contract,
                created_at: record.created_at,
                cannonical_name,
                meta,
            }));
        }
    }

    // NameRecord doesn't exist
    Ok(None)
}
