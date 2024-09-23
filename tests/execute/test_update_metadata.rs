#[cfg(test)]
mod test_update_metadata {
    use crate::test_utils::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor, IntoBech32};
    use cw_web31_dns::*;
    use error::*;
    use models::*;
    use msg::*;
    use token::TokenAmount;

    #[test]
    fn test_exec_update_metadata() {
        let name_owner_str = "name_owner".to_string();
        let owner_str = "owner".to_string();
        let amount = 1000u128;
        let mut app = def_app(owner_str.clone(), name_owner_str.clone(), amount);
        let contract = dns_contract();
        let code_id = app.store_code(contract);

        let owner_address = Addr::unchecked(owner_str.clone());
        let fee_recipient = Addr::unchecked("fee_recipient");
        let inst_msg = InstantiateMsg {
            price: TokenAmount {
                token: token::Token::Denom("juno".to_string()),
                amount: Uint128::from(1u128),
            },
            fee_recipient,
            max_name_len: 10,
        };

        let addr = app
            .instantiate_contract(
                code_id,
                owner_address.clone(),
                &inst_msg,
                &[],
                "test",
                Some(owner_address.to_string()),
            )
            .unwrap();

        let name_owner = Addr::unchecked(name_owner_str.clone());
        //register a name

        let name = "example".to_string();
        let site_address = "example_site_cw_address".into_bech32().to_string();
        let reg_msg = ExecuteMsg::Register(RegisterMsg {
            owner: name_owner.clone(),
            name: name.clone(),
            address: site_address,
            meta: None,
        });
        let _resp = app
            .execute_contract(name_owner.clone(), addr.clone(), &reg_msg, &coins(1, "juno"))
            .unwrap();

        //query the name metadata
        let query_msg = msg::QueryMsg::NameRecord { contract: name.clone() };
        let resp: PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
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
            },
        };
        let _resp = app
            .execute_contract(
                name_owner.clone(),
                addr.clone(),
                &ExecuteMsg::UpdateMetadata(update_msg),
                &[],
            )
            .unwrap();

        //query the name metadata
        let query_msg = msg::QueryMsg::NameRecord { contract: name.clone() };
        let resp: PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
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
            },
        };
        let _resp = app
            .execute_contract(
                name_owner.clone(),
                addr.clone(),
                &ExecuteMsg::UpdateMetadata(update_msg),
                &[],
            )
            .unwrap();

        //query the name metadata
        let query_msg = msg::QueryMsg::NameRecord { contract: name.clone() };
        let resp: PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(
            resp.meta.favicon,
            Some(ImageAsset::Url("example_favicon_url".to_string()))
        );
        assert_eq!(resp.meta.logo, Some(ImageAsset::Url("example_logo_url".to_string())));
        assert_eq!(
            resp.meta.keywords,
            Some(vec!["example_keyword1".to_string(), "example_keyword2".to_string()])
        );

        //clean the keywords metadata
        let update_msg = UpdateMetadataMsg {
            name: name.to_string(),
            meta: NameMetadata {
                title: None,
                description: None,
                favicon: None,
                logo: None,
                keywords: Some(vec![]),
            },
        };
        let _resp = app
            .execute_contract(
                name_owner.clone(),
                addr.clone(),
                &ExecuteMsg::UpdateMetadata(update_msg),
                &[],
            )
            .unwrap();

        //query the name metadata
        let query_msg = msg::QueryMsg::NameRecord { contract: name.clone() };
        let resp: PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(
            resp.meta.favicon,
            Some(ImageAsset::Url("example_favicon_url".to_string()))
        );
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
            },
        };
        let err = app
            .execute_contract(
                owner_address.clone(),
                addr.clone(),
                &ExecuteMsg::UpdateMetadata(update_msg),
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::NotAuthorized {
                reason: "You are not the owner of this name".to_string(),
            },
            err.downcast().unwrap()
        );

        //query the name metadata
        let query_msg = msg::QueryMsg::NameRecord { contract: name.clone() };
        let resp: PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(
            resp.meta.favicon,
            Some(ImageAsset::Url("example_favicon_url".to_string()))
        );
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
            },
        };
        let err = app
            .execute_contract(
                name_owner.clone(),
                addr.clone(),
                &ExecuteMsg::UpdateMetadata(update_msg),
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::ValidationError {
                reason: format!("Title must be less than {} characters", NameMetadata::MAX_TITLE_LEN),
            },
            err.downcast().unwrap()
        );

        //query the name metadata
        let query_msg = msg::QueryMsg::NameRecord { contract: name.clone() };
        let resp: PublicNameRecord = app.wrap().query_wasm_smart(addr.clone(), &query_msg).unwrap();
        print!("resp: {:?}\n", resp);
        assert_eq!(resp.meta.title, Some("example_title".to_string()));
        assert_eq!(resp.meta.description, Some("example_description".to_string()));
        assert_eq!(
            resp.meta.favicon,
            Some(ImageAsset::Url("example_favicon_url".to_string()))
        );
        assert_eq!(resp.meta.logo, Some(ImageAsset::Url("example_logo_url".to_string())));
        assert_eq!(resp.meta.keywords, Some(vec![]));
    }
}
