use crate::{
    json,
    lsp::json::{self, JSONObject, ToJson},
};

#[derive(Default)]
pub struct ClientCapabilities {
    workspace: Option<Workspace>,
    // did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,
}
impl ToJson for ClientCapabilities {
    fn to_json(self) -> super::JSONObject {
        json! {
            workspace: match self.workspace {
                Some(w) => json!(w),
                None => JSONObject::Null
            }
        }
    }
}

#[derive(Default)]
pub struct Workspace {
    apply_edit: Option<bool>,
    workspace_edit: Option<WorkspaceEditClientCapabilities>,
}
impl ToJson for Workspace {
    fn to_json(self) -> JSONObject {
        JSONObject::new()
            .rinsert("applyEdit", self.apply_edit)
            .rinsert_option("workspaceEdit", self.workspace_edit)
    }
}

#[derive(Default)]
pub struct WorkspaceEditClientCapabilities {
    document_changes: Option<bool>,
    resources_operations: Option<Vec<ResourceOperationKind>>,
    failure_handling: Option<FailureHandlingKind>,
    normalizes_line_endings: Option<bool>,
    change_annotation_support: Option<WorkspaceChangeAnnotationSupport>,
}
impl ToJson for WorkspaceEditClientCapabilities {
    fn to_json(self) -> JSONObject {
        JSONObject::new()
            .rinsert("documentChanges", self.document_changes)
            .rinsert_option("resourcesOperations", self.resources_operations)
            .rinsert_option("failureHandling", self.failure_handling)
            .rinsert_option("normalizesLineEndings", self.normalizes_line_endings)
            .rinsert_option("changeAnnotationSupport", self.change_annotation_support)
    }
}
impl ToJson for Vec<ResourceOperationKind> {
    fn to_json(self) -> JSONObject {
        JSONObject::Array(self.into_iter().map(|f| f.to_json()).collect())
    }
}

#[derive(Default)]
pub struct WorkspaceChangeAnnotationSupport {
    group_on_label: Option<bool>,
}
impl ToJson for WorkspaceChangeAnnotationSupport {
    fn to_json(self) -> JSONObject {
        JSONObject::new().rinsert_option("groupOnLabel", self.group_on_label)
    }
}

#[derive(Default)]
pub enum ResourceOperationKind {
    #[default]
    Create,
    Rename,
    Delete,
}
impl ToJson for ResourceOperationKind {
    fn to_json(self) -> JSONObject {
        match self {
            Self::Create => json!("create"),
            Self::Rename => json!("rename"),
            Self::Delete => json!("delete"),
        }
    }
}

#[derive(Default)]
pub enum FailureHandlingKind {
    #[default]
    Abort,
    Transactional,
    Undo,
    TextOnlyTransactional,
}
impl ToJson for FailureHandlingKind {
    fn to_json(self) -> JSONObject {
        match self {
            Self::Abort => json!("abort"),
            Self::Transactional => json!("transactional"),
            Self::Undo => json!("undo"),
            Self::TextOnlyTransactional => json!("textOnlyTransactional"),
        }
    }
}
