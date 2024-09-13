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
    let Context { deps, info ,..} = ctx;

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
                return Err(ContractError::NotFound { reason: format!("Name {cannonical_name} has no metadata!, please report to admins") });
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{contract::{execute, instantiate, query}, models::{ImageAsset, PublicNameRecord}, msg::RegisterMsg, token::TokenAmount};
    use cosmwasm_std::Uint128;
    use cw_multi_test::{App, Contract, ContractWrapper, Executor, IntoBech32};
    use cw_orch::prelude::Empty;
    use crate::msg::InstantiateMsg;
    use crate::msg::ExecuteMsg;
    use cosmwasm_std::coins;
    use crate::models::NameMetadata;
    use crate::msg::UpdateMetadataMsg;
    use cosmwasm_std::Addr;

    fn def_app(addr1:String, addr2: String, amount: u128) -> App {
        let app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(addr1), coins(amount, "juno"))
                .unwrap();
            router.bank.init_balance(storage, &Addr::unchecked(addr2), coins(amount, "juno"))
                .unwrap();
        });
        app
    }

    fn dns_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute,instantiate,query);
        Box::new(contract)
    }

    #[test]
    fn test_exec_update_metadata() {
        let name_owner_str = "name_owner".to_string();
        let owner_str = "owner".to_string();
        let amount = 1000u128;
        let mut app = def_app(owner_str.clone(),name_owner_str.clone(),amount);
        let contract = dns_contract();
        let code_id = app.store_code(contract);

        let owner_address = Addr::unchecked(owner_str.clone());
        let fee_recipient = Addr::unchecked("fee_recipient");
        let inst_msg = InstantiateMsg { price: TokenAmount {token: crate::token::Token::Denom("juno".to_string()),amount: Uint128::from(1u128)}
                                        , fee_recipient:fee_recipient, max_name_len: 10 };

        let addr = app.instantiate_contract(code_id, owner_address.clone() , &inst_msg, &[], "test", Some(owner_address.to_string())).unwrap();

        let name_owner = Addr::unchecked(name_owner_str.clone());
        //register a name
        
        let name = "example".to_string();
        let site_address = "example_site_cw_address".into_bech32().to_string();
        let reg_msg = ExecuteMsg::Register(RegisterMsg {owner:name_owner.clone(), name:name.clone(), address:site_address, meta:None});
        let _resp = app.execute_contract(name_owner.clone(), addr.clone(), &reg_msg, &coins(1,"juno")).unwrap();

        //query the name metadata
        let query_msg = crate::msg::QueryMsg::NameRecord{ contract: name.clone() };
        let resp:PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}", resp);
        assert_eq!(resp.meta.title, None);
        assert_eq!(resp.meta.description, None);
        assert_eq!(resp.meta.favicon, None);
        assert_eq!(resp.meta.logo, None);
        assert_eq!(resp.meta.keywords, None);

        //update the metadata
        let update_msg = UpdateMetadataMsg {
            name: name.to_string(),
            meta: NameMetadata {
                title: Some("example_title".to_string()),
                description: Some("example_description".to_string()),
                favicon: None,
                logo: None,
                keywords: None,
            }
        };
        let _resp = app.execute_contract(name_owner.clone(), addr.clone(),
         &ExecuteMsg::UpdateMetadata(update_msg), &[]).unwrap();

        //query the name metadata
        let query_msg = crate::msg::QueryMsg::NameRecord{ contract: name.clone() };
        let resp:PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(resp.meta.favicon, None);
        assert_eq!(resp.meta.logo, None);
        assert_eq!(resp.meta.keywords, None);


        //update the metadata
        let update_msg = UpdateMetadataMsg {
            name: name.to_string(),
            meta: NameMetadata {
                title: None,
                description: None,
                favicon: Some(ImageAsset::Url("example_favicon_url".to_string())),
                logo: Some(ImageAsset::Url("example_logo_url".to_string())),
                keywords: Some(vec!["example_keyword1".to_string(), "example_keyword2".to_string()]),
            }
        };
        let _resp = app.execute_contract(name_owner.clone(), addr.clone(),
         &ExecuteMsg::UpdateMetadata(update_msg), &[]).unwrap();

        //query the name metadata
        let query_msg = crate::msg::QueryMsg::NameRecord{ contract: name.clone() };
        let resp:PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(resp.meta.favicon, Some(ImageAsset::Url("example_favicon_url".to_string())));
        assert_eq!(resp.meta.logo, Some(ImageAsset::Url("example_logo_url".to_string())));
        assert_eq!(resp.meta.keywords, Some(vec!["example_keyword1".to_string(), "example_keyword2".to_string()]));

        //clean the keywords metadata
        let update_msg = UpdateMetadataMsg {
            name: name.to_string(),
            meta: NameMetadata {
                title: None,
                description: None,
                favicon: None,
                logo: None,
                keywords: Some(vec![]),
            }
        };
        let _resp = app.execute_contract(name_owner.clone(), addr.clone(),
         &ExecuteMsg::UpdateMetadata(update_msg), &[]).unwrap();

        //query the name metadata
        let query_msg = crate::msg::QueryMsg::NameRecord{ contract: name.clone() };
        let resp:PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(resp.meta.favicon, Some(ImageAsset::Url("example_favicon_url".to_string())));
        assert_eq!(resp.meta.logo, Some(ImageAsset::Url("example_logo_url".to_string())));
        assert_eq!(resp.meta.keywords, Some(vec![]));


        //check error when updating metadata of non owner
        let update_msg = UpdateMetadataMsg {
            name: name.to_string(),
            meta: NameMetadata {
                title: None,
                description: None,
                favicon: None,
                logo: None,
                keywords: Some(vec![]),
            }
        };
        let err = app.execute_contract(owner_address.clone(), addr.clone(),
         &ExecuteMsg::UpdateMetadata(update_msg), &[]).unwrap_err();

        assert_eq!(ContractError::NotAuthorized{
            reason: "You are not the owner of this name".to_string(),
        },err.downcast().unwrap());

        //query the name metadata
        let query_msg = crate::msg::QueryMsg::NameRecord{ contract: name.clone() };
        let resp:PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(resp.meta.favicon, Some(ImageAsset::Url("example_favicon_url".to_string())));
        assert_eq!(resp.meta.logo, Some(ImageAsset::Url("example_logo_url".to_string())));
        assert_eq!(resp.meta.keywords, Some(vec![]));

        //check title length validation
        let update_msg = UpdateMetadataMsg {
            name: name.to_string(),
            meta: NameMetadata {
                title: Some("a".repeat(NameMetadata::MAX_TITLE_LEN + 1)),
                description: None,
                favicon: None,
                logo: None,
                keywords: None,
            }
        };
        let err = app.execute_contract(name_owner.clone(), addr.clone(),
         &ExecuteMsg::UpdateMetadata(update_msg), &[]).unwrap_err();

        assert_eq!(ContractError::ValidationError{
            reason: format!("Title must be less than {} characters", NameMetadata::MAX_TITLE_LEN),
        },err.downcast().unwrap());

        //query the name metadata
        let query_msg = crate::msg::QueryMsg::NameRecord{ contract: name.clone() };
        let resp:PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(resp.meta.favicon, Some(ImageAsset::Url("example_favicon_url".to_string())));
        assert_eq!(resp.meta.logo, Some(ImageAsset::Url("example_logo_url".to_string())));
        assert_eq!(resp.meta.keywords, Some(vec![]));

    }
}
