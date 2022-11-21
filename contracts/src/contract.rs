use abstract_extension::{export_endpoints, ExtensionContract};
use abstract_sdk::Execution;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Empty};

use crate::error::TemplateError;
use crate::handlers;
use crate::msg::{TemplateExecuteMsg, TemplateQueryMsg};

/// The namespace for the app, like "abstract" -> "abstract:template"
pub const MODULE_NAMESPACE: &str = "template_namespace";
/// The name of the app, excluding the namespace
pub const MODULE_NAME: &str = "template_app_name";
/// The initial version of the app, which will use the package version if not altered
const MODULE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type TemplateExtension = ExtensionContract<TemplateError, TemplateExecuteMsg, Empty, TemplateQueryMsg>;
pub type TemplateResult = Result<Response, TemplateError>;

const EXTENSION: TemplateExtension =
    TemplateExtension::new(MODULE_NAME, MODULE_VERSION)
        .with_execute(handlers::execute_handler)
        .with_query(handlers::query_handler)
        .with_replies(&[(1, handlers::example_reply_handler)]);


// don't export endpoints when imported as library
#[cfg(not(feature = "library"))]
// Export the endpoints for this contract
export_endpoints!(EXTENSION, TemplateExtension);
