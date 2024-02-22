use tower_lsp::Client;

#[derive(Debug, Clone)]
pub struct LanguageServer {
    pub client: Client,
}
