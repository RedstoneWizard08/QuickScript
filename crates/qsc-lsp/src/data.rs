use tower_lsp::lsp_types::{notification::Notification, Url};

use crate::hint::InlayHintParams;

pub enum CustomNotification {}

impl Notification for CustomNotification {
    type Params = InlayHintParams;
    const METHOD: &'static str = "custom/notification";
}

pub struct TextDocumentItem {
    pub uri: Url,
    pub text: String,
    pub version: i32,
}
