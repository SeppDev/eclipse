use crate::{
    json,
    lsp::{
        self,
        json::{JSONObject, ToJson},
    },
};

#[derive(Default)]
pub struct ServerCapabilities {
    position_encoding: Option<PositionEncodingKind>,
    text_document_sync: Option<TextDocumentSyncOptions>,
    completion_provider: Option<CompletionOptions>,
}
impl ToJson for ServerCapabilities {
    fn to_json(self) -> JSONObject {
        JSONObject::new()
            .rinsert_option("positionEncoding", self.position_encoding)
            .rinsert_option("textDocumentSync", self.text_document_sync)
            .rinsert_option("completionProvider", self.completion_provider)
    }
}

pub struct CompletionOptions {
    trigger_characters: Option<Vec<char>>,
    all_commit_characters: Option<Vec<char>>,
    resolve_provider: Option<bool>,
    completion_item: Option<CompletionItem>,
}
impl ToJson for CompletionOptions {
    fn to_json(self) -> JSONObject {
        JSONObject::new()
            .rinsert_option("triggerCharacters", self.trigger_characters)
            .rinsert_option("allCommitCharacters", self.all_commit_characters)
            .rinsert_option("resolveProvider", self.resolve_provider)
            .rinsert_option("completionItem", self.completion_item)
    }
}
impl ToJson for Vec<char> {
    fn to_json(self) -> JSONObject {
        lsp::json::common::vec_to_json(self)
    }
}

pub struct CompletionItem {
    label_detail_support: Option<bool>,
}
impl ToJson for CompletionItem {
    fn to_json(self) -> JSONObject {
        JSONObject::new().rinsert("labelDetailSupport", self.label_detail_support)
    }
}

pub struct TextDocumentSyncOptions {
    open_close: Option<bool>,
    change: Option<TextDocumentSyncKind>,
}
impl ToJson for TextDocumentSyncOptions {
    fn to_json(self) -> JSONObject {
        JSONObject::new()
            .rinsert_option("openClose", self.open_close)
            .rinsert_option("change", self.change)
    }
}

pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}
impl ToJson for TextDocumentSyncKind {
    fn to_json(self) -> JSONObject {
        json!(self as usize)
    }
}

pub enum PositionEncodingKind {
    UTF8,
    UTF16,
    UTF32,
}
impl ToJson for PositionEncodingKind {
    fn to_json(self) -> JSONObject {
        match self {
            Self::UTF8 => json!("utf-8"),
            Self::UTF16 => json!("utf-16"),
            Self::UTF32 => json!("utf-32"),
        }
    }
}
