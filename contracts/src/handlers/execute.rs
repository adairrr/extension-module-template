//! # Staking
use cosmwasm_std::{Addr, Api, Coin, CosmosMsg, DepsMut, DistributionMsg, Env, MessageInfo, QuerierWrapper, Response, StakingMsg, StdError, StdResult};
use abstract_sdk::Execution;
use crate::contract::{TemplateExtension, TemplateResult};
use crate::msg::TemplateExecuteMsg;

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    extension: TemplateExtension,
    msg: TemplateExecuteMsg,
) -> TemplateResult {

    // Use executor to execute arbirtrary CosmosMsgs
    let executor = extension.executor(deps.as_ref());
    let msg = match msg {
        TemplateExecuteMsg::Todo { } => {
            executor.execute(vec![])
        }

    }?;
    Ok(Response::new().add_message(msg))
}


pub fn delegate_to(
    querier: &QuerierWrapper,
    validator: &str,
    amount: u128,
) -> StdResult<CosmosMsg> {
    let denom = querier.query_bonded_denom()?;
    Ok(CosmosMsg::Staking(StakingMsg::Delegate {
        validator: validator.to_string(),
        amount: Coin::new(amount, denom),
    }))
}
