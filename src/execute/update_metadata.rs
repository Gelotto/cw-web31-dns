use crate::{
    error::ContractError,
    msg::UpdateMetadataMsg,
    state::{NAME_METADATA, NAME_RECORDS},
};
use cosmwasm_std::Response;

use super::Context;

pub fn exec_update_metadata(
    ctx: Context,
    msg: UpdateMetadataMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    let UpdateMetadataMsg { name, meta } = msg;

    meta.validate()?;

    let cannonical_name = name.to_ascii_lowercase();

    // Ensure the name record exists
    let record = NAME_RECORDS.load(deps.storage, &cannonical_name)?;

    // Ensure the caller is the owner of the name record
    if record.owner != info.sender {
        return Err(ContractError::NotAuthorized {
            reason: "You are not the owner of this name".to_string(),
        });
    }

    // Update the metadata, if the record already exists substitute the new metadata fields
    NAME_METADATA.update(
        deps.storage,
        &cannonical_name,
        |maybe_meta| -> Result<_, ContractError> {
            if maybe_meta.is_none() {
                return Err(ContractError::NotFound {
                    reason: format!("Name {cannonical_name} has no metadata!, please report to admins"),
                });
            }
            let mut new_meta = maybe_meta.unwrap();
            new_meta.title = meta.title.or(new_meta.title);
            new_meta.description = meta.description.or(new_meta.description);
            new_meta.favicon = meta.favicon.or(new_meta.favicon);
            new_meta.logo = meta.logo.or(new_meta.logo);
            new_meta.keywords = meta.keywords.or(new_meta.keywords);
            Ok(new_meta)
        },
    )?;

    // NAME_METADATA.save(deps.storage, &cannonical_name, &meta)?;

    Ok(Response::new().add_attribute("action", "update_metadata"))
}
