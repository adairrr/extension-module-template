use abstract_os::app::{BaseExecuteMsg, BaseQueryMsg};
use abstract_os::{base, extension};
use boot_core::prelude::boot_contract;

use boot_core::{BootEnvironment, BootError, Contract, IndexResponse, TxHandler, TxResponse};
use cosmwasm_std::Coin;
use serde::de::DeserializeOwned;
use template_app::contract::MODULE_NAME;
use template_app::msg::{
    TemplateExecuteMsg, TemplateQueryMsg,
};
use cosmwasm_std::Empty;

type ExtensionExecuteMsg = extension::ExecuteMsg<TemplateExecuteMsg>;
type ExtensionQueryMsg = extension::QueryMsg<TemplateQueryMsg>;

/// Contract wrapper for interacting with BOOT
#[boot_contract(Empty, ExtensionExecuteMsg, ExtensionQueryMsg, Empty)]
pub struct TemplateExtension<Chain>;

/// implement chain-generic functions
impl<Chain: BootEnvironment> TemplateExtension<Chain> where TxResponse<Chain>: IndexResponse {
    pub fn new(id: &str, chain: &Chain) -> Self {
        Self(
            Contract::new(id, chain)
                .with_wasm_path(MODULE_NAME),
        )
    }
}
