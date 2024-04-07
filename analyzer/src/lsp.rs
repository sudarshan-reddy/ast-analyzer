use anyhow::Result;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::str::from_utf8;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, warn, Level};
use tracing_subscriber;

pub struct LspClient {
    stream: TcpStream,
}

impl LspClient {
    pub async fn new(addr: &str) -> Result<LspClient> {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();
        info!("Connecting to LSP server at: {}", addr);
        let stream = TcpStream::connect(addr).await?;
        Ok(LspClient { stream })
    }

    pub async fn send_initialize_request(&mut self, project_path: &str) -> Result<()> {
        // Ensure project_path is absolute
        if !PathBuf::from(project_path).is_absolute() {
            return Err(anyhow::anyhow!("Project path must be absolute"));
        };
        let root_uri = format!("file://{}", project_path.to_string());
        // TODO: Support multi root workspaces
        let workspace_name = root_uri.clone();

        info!("Sending initialize request with root URI: {}", root_uri);

        let initialize_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "processId": std::process::id(),
                "clientInfo": {
                    "name": "YourLSPClientName", // Customize this
                    "version": "1.0.0" // Optional: Adjust as necessary
                },
                "rootUri": root_uri,
                "capabilities": {
                    "workspace": {
                        "workspaceFolders": true,
                        "didChangeConfiguration": {
                            "dynamicRegistration": true
                        },
                        "workspaceEdit": {
                            "documentChanges": true
                        },
                        "configuration": true
                    },
                    "textDocument": {
                        "hover": {
                            "contentFormat": ["plaintext"]
                        },
                        "completion": {
                            "completionItem": {
                                "snippetSupport": true // Set to false if your client does not support snippets
                            }
                        },
                        "codeAction": {
                            "codeActionLiteralSupport": {
                                "codeActionKind": {
                                    "valueSet": ["source.organizeImports", "refactor.rewrite", "refactor.extract"]
                                }
                            }
                        }
                    }
                },
                "workspaceFolders": [{
                    "uri": root_uri,
                    "name": workspace_name ,
                }]
            }
        });

        self.send_request(&initialize_request).await?;
        let init_resp = self.read_response(None).await?;
        info!("Initialize response: {:?}", from_utf8(&init_resp));

        let initialized_notification = json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        });

        self.send_request(&initialized_notification).await?;
        info!("Sent initialized notification");

        Ok(())
    }

    pub async fn get_definition(
        &mut self,
        file_path: &str,
        line: u32,
        character: u32,
    ) -> Result<()> {
        if !PathBuf::from(file_path).is_absolute() {
            return Err(anyhow::anyhow!("File path must be absolute"));
        }

        info!(
            "Sending definition request for file: {}, line: {}, col: {} ",
            file_path, line, character
        );

        let definition_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/definition",
            "notification": 0,
            "params": {
                "textDocument": {
                    "uri": format!("file://{}", file_path),
                },
                "position": {
                    "line": line,
                    "character": character,
                },
            }
        });

        self.send_request(&definition_request).await?;
        let def = self.read_response(Some(2)).await?;
        info!("Definition: {:?}", from_utf8(&def));
        Ok(())
    }

    async fn send_request(&mut self, request: &serde_json::Value) -> Result<()> {
        let request_str = request.to_string();
        let content_length = request_str.as_bytes().len();
        let header = format!("Content-Length: {}\r\n\r\n{}", content_length, request_str);

        self.stream.write_all(header.as_bytes()).await?;
        self.stream.flush().await?;
        Ok(())
    }

    async fn read_response(&mut self, expected_id: Option<u64>) -> Result<Vec<u8>> {
        loop {
            let mut header_buffer = Vec::new();
            // Read headers
            while !header_buffer.ends_with(b"\r\n\r\n") {
                let mut buffer = [0; 1]; // Read one byte at a time
                self.stream.read_exact(&mut buffer).await?;
                header_buffer.push(buffer[0]);
            }

            // Convert headers to string and find Content-Length
            let header_str = String::from_utf8(header_buffer)?;
            let content_length = header_str
                .lines()
                .find_map(|line| {
                    if line.starts_with("Content-Length:") {
                        line["Content-Length:".len()..].trim().parse::<usize>().ok()
                    } else {
                        None
                    }
                })
                .ok_or_else(|| anyhow::anyhow!("Content-Length header not found"))?;

            // Read the message body based on Content-Length
            let mut response_body = vec![0; content_length];
            self.stream.read_exact(&mut response_body).await?;

            // Attempt to deserialize the response to check its ID
            if let Ok(response) = serde_json::from_slice::<Value>(&response_body) {
                // If no specific ID is expected, or if this message matches the expected ID, return it
                if expected_id.is_none()
                    || response.get("id") == expected_id.map(serde_json::Value::from).as_ref()
                {
                    return Ok(response_body);
                } else {
                    // Log or handle interim messages
                    info!("Interim or unrelated LSP message received: {:?}", response);
                    // Continue looping to wait for the correct response
                }
            } else {
                // Handle or log parsing error
                warn!("Failed to parse LSP response as JSON.");
                // Depending on your error handling strategy, you might return an error here
            }
        }
    }
}
