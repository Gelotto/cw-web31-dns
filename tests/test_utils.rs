use contract::{execute, instantiate, query};
use cosmwasm_std::{coins, Addr};
use cw_multi_test::{App, Contract, ContractWrapper};
use cw_orch::prelude::Empty;
use cw_web31_dns::{contract, models, msg, token};

pub fn def_app(
    addr1: String,
    addr2: String,
    amount: u128,
) -> App {
    let app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked(addr1), coins(amount, "juno"))
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked(addr2), coins(amount, "juno"))
            .unwrap();
    });
    app
}

pub fn dns_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}
