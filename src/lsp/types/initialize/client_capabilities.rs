#[derive(Default)]
pub struct ClientCapabilities {
    workspace: Option<WorkspaceClientCapabilities>,
    // did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,
}

#[derive(Default)]
pub struct WorkspaceClientCapabilities {
    apply_edit: Option<bool>,
}

#[derive(Default)]
pub struct WorkspaceEditClientCapabilities {
    document_changes: Option<bool>,
    resources_operations: Option<Vec<ResourceOperationKind>>,
    failure_handling: Option<FailureHandlingKind>,
    normalizes_line_endings: Option<bool>,
    change_annotation_support: Option<WorkspaceChangeAnnotationSupport>,
}

#[derive(Default)]
pub struct WorkspaceChangeAnnotationSupport {
    group_on_label: Option<bool>,
}

#[derive(Default)]
pub enum ResourceOperationKind {
    #[default]
    Create,
    Rename,
    Delete,
}

#[derive(Default)]
pub enum FailureHandlingKind {
    #[default]
    Abort,
    Transactional,
    Undo,
    TextOnlyTransactional,
}
