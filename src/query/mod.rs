pub mod config;
pub mod name_record;
pub mod name_records;
pub mod render;

use cosmwasm_std::{Deps, Env};

pub struct ReadonlyContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}
