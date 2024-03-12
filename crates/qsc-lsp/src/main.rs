use dashmap::DashMap;
use qsc_lsp::backend::Backend;
use tokio::io::{stdin, stdout};
use tower_lsp::{LspService, Server};

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        ast_map: DashMap::new(),
        document_map: DashMap::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
