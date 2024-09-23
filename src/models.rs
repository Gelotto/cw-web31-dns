use super::query::ReadonlyContext;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

use crate::{error::ContractError, state::NAME_METADATA};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct NameRecord {
    pub owner: Addr,
    pub contract: String,
    pub created_at: Timestamp,
}

#[cw_serde]
#[derive(Default)]
pub struct NameMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub favicon: Option<ImageAsset>,
    pub logo: Option<ImageAsset>,
    pub keywords: Option<Vec<String>>,
}

#[cw_serde]
pub enum ImageAsset {
    Svg(String),
    Url(String),
}

#[cw_serde]
pub struct PublicNameRecord {
    pub owner: Addr,
    pub cannonical_name: String,
    pub contract: String,
    pub created_at: Timestamp,
    pub meta: NameMetadata,
}

//add validate function to NameMetadata struct to be moved in ????
impl NameMetadata {
    pub const MAX_TITLE_LEN: usize = 100;
    pub const MAX_DESCRIPTION_LEN: usize = 500;
    pub const MAX_KEYWORDS: usize = 10;
    pub const MAX_KEYWORD_LEN: usize = 50;

    pub fn validate(&self) -> Result<(), ContractError> {
        if let Some(title) = &self.title {
            if title.len() > Self::MAX_TITLE_LEN {
                return Err(ContractError::ValidationError {
                    reason: format!("Title must be less than {} characters", Self::MAX_TITLE_LEN),
                });
            }
        }
        if let Some(description) = &self.description {
            if description.len() > Self::MAX_DESCRIPTION_LEN {
                return Err(ContractError::ValidationError {
                    reason: format!("Description must be less than {} characters", Self::MAX_DESCRIPTION_LEN),
                });
            }
        }
        if let Some(keywords) = &self.keywords {
            if keywords.len() > Self::MAX_KEYWORDS {
                return Err(ContractError::ValidationError {
                    reason: format!("Keywords must be less than {} in length", Self::MAX_KEYWORDS),
                });
            }
            for keyword in keywords {
                if keyword.len() > Self::MAX_KEYWORD_LEN {
                    return Err(ContractError::ValidationError {
                        reason: format!("Keyword must be less than {} characters", Self::MAX_KEYWORD_LEN),
                    });
                }
            }
        }
        Ok(())
    }
}

impl NameRecord {
    pub fn build_public_name_record(
        &self,
        ctx: &ReadonlyContext,
        cannonical_name: String,
    ) -> Result<PublicNameRecord, ContractError> {
        let ReadonlyContext { deps, .. } = ctx;

        let meta = NAME_METADATA.load(deps.storage, &cannonical_name)?;
        return Ok(PublicNameRecord {
            owner: self.owner.clone(),
            contract: self.contract.clone(),
            created_at: self.created_at,
            cannonical_name,
            meta,
        });
    }
}
