//! # Tendermint Staking Extension
//!
//! `abstract_os::tendermint_staking` exposes all the function of [`cosmwasm_std::CosmosMsg::Staking`] and [`cosmwasm_std::CosmosMsg::Distribution`].

use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Uint128;

#[cosmwasm_schema::cw_serde]
pub enum TemplateExecuteMsg {
    Todo {},
}

/// Staking queries are available on [`cosmwasm_std::QuerierWrapper`] through [`cosmwasm_std::Deps`]. Helper function are exposed by [`abstract_sdk::TODOtendermint_staking`]
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses)]
pub enum TemplateQueryMsg {
    /// Returns [`TodoResponse`]
    #[returns(TodoResponse)]
    Todo {},
}

#[cosmwasm_schema::cw_serde]
pub struct TodoResponse {}
