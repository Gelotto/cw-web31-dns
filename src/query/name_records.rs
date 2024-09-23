use std::marker::PhantomData;

use super::ReadonlyContext;
use crate::msg::{NameRecordsQueryMsg, NameRecordsQueryResponse};
use crate::{error::ContractError, models::PublicNameRecord, state::NAME_RECORDS};
use cw_storage_plus::Bound;

pub const MAX_REQUEST_LIMIT: u8 = 30;
/// Return x number of NameRecords
pub fn query_name_records(
    ctx: ReadonlyContext,
    msg: NameRecordsQueryMsg,
) -> Result<Option<NameRecordsQueryResponse>, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let NameRecordsQueryMsg {
        cursor,
        limit,
        network_prefix: network_den,
    } = msg;

    if limit > MAX_REQUEST_LIMIT {
        return Err(ContractError::TooManyRecords {
            limit: MAX_REQUEST_LIMIT,
        });
    }

    let bounding_id: String;
    let mut min_bound: Option<Bound<&String>> = None;
    // check if the cursor is not none and exists
    if let Some(id) = cursor {
        if NAME_RECORDS.may_load(deps.storage, &id)?.is_none() {
            return Err(ContractError::NotFound {
                reason: format!("Name {} not found", id.clone()),
            });
        }
        bounding_id = id.clone();
        min_bound = Some(Bound::Exclusive((&bounding_id, PhantomData)));
    }

    let max_bound = None;

    let network = network_den.unwrap_or("".to_string());

    let mut name_records: Vec<PublicNameRecord> = Vec::with_capacity(limit as usize);

    let mut next_cursor_id = None;
    for record in NAME_RECORDS
        .range(deps.storage, min_bound, max_bound, cosmwasm_std::Order::Ascending)
        .filter(|item| {
            let (_id, name_record_item) = item.as_ref().unwrap();
            name_record_item.contract.starts_with(&network)
        })
        .take(limit as usize)
    {
        let (name, name_record) = record?;
        next_cursor_id = Some(name.clone());
        let public_name_record = name_record.build_public_name_record(&ctx, name)?;
        name_records.push(public_name_record);
    }
    Ok(Some(NameRecordsQueryResponse {
        name_records,
        next_cursor: next_cursor_id,
    }))
}
