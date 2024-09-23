#[cfg(test)]
mod test_name_records {
    use crate::test_utils::*;
    use cosmwasm_std::{coins, StdError};
    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor, IntoBech32};
    use cw_web31_dns::*;
    use error::*;
    use models::*;
    use msg::*;
    use token::TokenAmount;

    #[test]
    fn test_query_name_records() {
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
        // register 10 names
        let network_den = "juno";
        for i in 0..10 {
            let name = format!("example{}", i);
            let site_address = format!("example_site_address{}", i)
                .into_bech32_with_prefix(&network_den)
                .to_string();
            println!("site_address: {:?}", site_address);
            let reg_msg = ExecuteMsg::Register(RegisterMsg {
                owner: name_owner.clone(),
                name: name.clone(),
                address: site_address.clone(),
                meta: Some(models::NameMetadata {
                    title: Some(format!("example_title{}",i)),
                    description: Some(format!("example_description{}",i)),
                    favicon: Some(ImageAsset::Url(format!("example_favicon_url{}",i))),
                    logo: Some(ImageAsset::Url(format!("example_logo_url{}",i))),
                    keywords: Some(vec![format!("example_keyword{}",i)]),
                }),
            });
            let _resp = app
                .execute_contract(name_owner.clone(), addr.clone(), &reg_msg, &coins(1, "juno"))
                .unwrap();
        }
        //query the name metadata
        let query_msg = msg::QueryMsg::NameRecords(NameRecordsQueryMsg {
            limit: 10,
            cursor: None,
            network_prefix: Some("juno".to_string()),
        });
        let resp = app.wrap().query_wasm_smart(addr.clone(), &query_msg);
        let name_records_msg: NameRecordsQueryResponse = resp.unwrap();
        let next_cursor = name_records_msg.next_cursor.clone();
        match serde_json::to_string_pretty(&name_records_msg) {
            Ok(json) => println!("name_records_msg: {}", json),
            Err(e) => println!("Failed to serialize to JSON: {}", e),
        }
        assert_eq!(name_records_msg.name_records.len(), 10);
        //check all the names
        for i in 0..10 {
            let name = format!("example{}", i);
            let site_address = format!("example_site_address{}", i)
                .into_bech32_with_prefix(&network_den)
                .to_string();
            assert_eq!(name_records_msg.name_records[i].cannonical_name, name);
            assert_eq!(name_records_msg.name_records[i].contract, site_address);
        }
        //query the name metadata with cursor expecting an empty array
        let query_msg = msg::QueryMsg::NameRecords(NameRecordsQueryMsg {
            limit: 10,
            cursor: next_cursor,
            network_prefix: None,
        });
        let resp = app.wrap().query_wasm_smart(addr.clone(), &query_msg);
        let name_records_msg: NameRecordsQueryResponse = resp.unwrap();
        match serde_json::to_string_pretty(&name_records_msg) {
            Ok(json) => println!("name_records_msg: {}", json),
            Err(e) => println!("Failed to serialize to JSON: {}", e),
        }
        assert_eq!(name_records_msg.name_records.len(), 0);
        assert_eq!(name_records_msg.next_cursor, None);

        //query the names expecting 5 names
        let query_msg = msg::QueryMsg::NameRecords(NameRecordsQueryMsg {
            limit: 5,
            cursor: None,
            network_prefix: None,
        });
        let resp = app.wrap().query_wasm_smart(addr.clone(), &query_msg);
        let name_records_msg: NameRecordsQueryResponse = resp.unwrap();
        let next_cursor = name_records_msg.next_cursor.clone();
        match serde_json::to_string_pretty(&name_records_msg) {
            Ok(json) => println!("name_records_msg: {}", json),
            Err(e) => println!("Failed to serialize to JSON: {}", e),
        }
        assert_eq!(name_records_msg.name_records.len(), 5);

        //query the names expecting the last 5 names
        let query_msg = msg::QueryMsg::NameRecords(NameRecordsQueryMsg {
            limit: 10,
            cursor: next_cursor,
            network_prefix: None,
        });
        let resp = app.wrap().query_wasm_smart(addr.clone(), &query_msg);
        let name_records_msg: NameRecordsQueryResponse = resp.unwrap();
        // let next_cursor = name_records_msg.next_cursor.clone();
        match serde_json::to_string_pretty(&name_records_msg) {
            Ok(json) => println!("name_records_msg: {}", json),
            Err(e) => println!("Failed to serialize to JSON: {}", e),
        }
        assert_eq!(name_records_msg.name_records.len(), 5);
        for i in 5..10 {
            let name = format!("example{}", i);
            let site_address = format!("example_site_address{}", i)
                .into_bech32_with_prefix(&network_den)
                .to_string();
            assert_eq!(name_records_msg.name_records[i - 5].cannonical_name, name);
            assert_eq!(name_records_msg.name_records[i - 5].contract, site_address);
        }

        // query the names with another network_den expecting 0 names
        let query_msg = msg::QueryMsg::NameRecords(NameRecordsQueryMsg {
            limit: 10,
            cursor: None,
            network_prefix: Some("osmo".to_string()),
        });
        let resp = app.wrap().query_wasm_smart(addr.clone(), &query_msg);
        let name_records_msg: NameRecordsQueryResponse = resp.unwrap();
        // let next_cursor = name_records_msg.next_cursor.clone();
        match serde_json::to_string_pretty(&name_records_msg) {
            Ok(json) => println!("name_records_msg: {}", json),
            Err(e) => println!("Failed to serialize to JSON: {}", e),
        }
        assert_eq!(name_records_msg.name_records.len(), 0);

        //query the name metadata with error TooManyRecords
        let query_msg = msg::QueryMsg::NameRecords(NameRecordsQueryMsg {
            limit: 31,
            cursor: None,
            network_prefix: None,
        });
        let resp: Result<NameRecordsQueryResponse, StdError> = app.wrap().query_wasm_smart(addr.clone(), &query_msg);
        println!("resp: {:?}", resp);
        assert_eq!(resp.is_err(), true);
        // Extract the error message from StdError
        let err_msg = resp.err().unwrap().to_string();
        assert_eq!(err_msg, format!("Generic error: Querier contract error: TooManyRecords: Too many records requested. Maximum Limit is {}", query::name_records::MAX_REQUEST_LIMIT));

        //query the name metadata with error NotFound
        let query_msg = msg::QueryMsg::NameRecords(NameRecordsQueryMsg {
            limit: 10,
            cursor: Some("not_added".to_string()),
            network_prefix: None,
        });
        let resp: Result<NameRecordsQueryResponse, StdError> = app.wrap().query_wasm_smart(addr.clone(), &query_msg);
        println!("resp: {:?}", resp);
        assert_eq!(resp.is_err(), true);
        // Extract the error message from StdError
        let err_msg = resp.err().unwrap().to_string();
        assert!(err_msg.contains("NotFound:"));
    }
}
