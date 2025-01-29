use crate::{json, lsp::json::ToJson};

use super::{DocumentURI, JSONObject, LSPAny, ValidOrNull, URI};

mod client_capabilities;
mod server_capabilities;
pub use client_capabilities::*;
pub use server_capabilities::*;

pub struct InitializeParams {
    process_id: ValidOrNull<isize>,
    client_info: Option<ClientInfo>,
    locale: Option<String>,
    root_uri: ValidOrNull<DocumentURI>,
    initialization_options: Option<LSPAny>,
    capabilities: ClientCapabilities,
    trace: Option<TraceValue>,
    workspace_folders: ValidOrNull<Vec<WorkspaceFolder>>,
}

#[derive(Default)]
pub enum TraceValue {
    #[default]
    Off,
    Messages,
    Verbose,
}
impl ToJson for TraceValue {
    fn to_json(self) -> JSONObject {
        match self {
            Self::Off => json!("off"),
            Self::Messages => json!("messages"),
            Self::Verbose => json!("verbose"),
        }
    }
}

pub struct WorkspaceFolder {
    uri: URI,
    name: String,
}

pub struct ClientInfo {
    name: String,
    version: Option<String>,
}
