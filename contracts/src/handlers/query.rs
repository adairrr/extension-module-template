use cosmwasm_std::{Binary, Deps, Env, StdResult, to_binary, Uint128};


use crate::contract::{TemplateExtension};
use crate::msg::{TemplateQueryMsg, TodoResponse};

const DEFAULT_PAGE_SIZE: u8 = 5;
const MAX_PAGE_SIZE: u8 = 20;

/// Handle queries sent to this extension.
pub fn query_handler(
    _deps: Deps,
    _env: Env,
    _app: &TemplateExtension,
    msg: TemplateQueryMsg,
) -> StdResult<Binary> {
    match msg {
        TemplateQueryMsg::Todo {} => to_binary(&TodoResponse {}),
    }
}
