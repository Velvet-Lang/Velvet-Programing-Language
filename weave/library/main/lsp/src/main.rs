use tower_lsp::{LspService, Server};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../core/velvet.pest"]
struct VelvetParser;

#[derive(Default)]
struct Backend;

#[tower_lsp::async_trait]
impl tower_lsp::LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        println!("Velvet LSP initialized");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        match VelvetParser::parse(Rule::program, &text) {
            Ok(_) => println!("Parsed document: {}", params.text_document.uri),
            Err(e) => println!("Parse error: {}", e),
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|_| Backend);
    Server::new(stdin, stdout, socket).serve(service).await;
}
