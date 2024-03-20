use tree_sitter::{Language, Parser, Node};
use anyhow::Result;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use std::sync::mpsc::{Sender, Receiver};

pub struct LangParser {
    pub language : Language,
    pub parser : Parser,
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

impl LangParser {
    pub fn new() ->  Result<LangParser> {
        let language = tree_sitter_go::language();
        let mut parser = Parser::new();
        let (tx, rx) = std::sync::mpsc::channel();
        parser.set_language(language)?;
        Ok(LangParser {
            language,
            parser,
            tx,
            rx,
        })
    }

    pub fn parse(&mut self, code: &str) -> anyhow::Result<tree_sitter::Tree> {
        let parsed = self.parser.parse(code, None).ok_or(anyhow::anyhow!("Error parsing code"))?;
        Ok(parsed)
    }

    pub fn traverse(&self, node: &Node, source_code: &[u8]) {
        match node.kind() {
            // For struct type nodes, print the full struct declaration
            "type_spec" if node.child_by_field_name("type").map_or(false, |n| n.kind() == "struct_type") => {
                let text = self.get_node_text(node, source_code);
                self.tx.send(text).expect("Error sending struct");
            },
            // For method declaration nodes, print the full method including its body
            "method_declaration" => {
                let text = self.get_node_text(node, source_code);
                self.tx.send(text).expect("Error sending method");
            },
            _ => {}
        }

        // Recursively traverse child nodes
        for child in node.children(&mut node.walk()) {
            self.traverse(&child, source_code);
        }
    }

    // Helper function to extract the text for a given node
    pub fn get_node_text(&self, node: &Node, source_code: &[u8]) -> String {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let text = &source_code[start_byte..end_byte];

        String::from_utf8(text.to_vec()).expect("Found invalid UTF-8")
    }

}


pub struct LLMClient {
    inner: Ollama
}

impl LLMClient {
    pub fn new() -> LLMClient {
        let inner = Ollama::default();
        LLMClient {
            inner
        }
    }

     pub async fn get(&self, language: &str, code: &str) -> Result<String> {
        let model = "codellama".to_string();
        // TODO: Improve this prompt
        let prompt = format!(r#"Add a detailed doc comment to the following {} method:
            {}
            The doc comment should describe what the method does. 
            Return the method implementaion with the doc comment above the method
            as a code comment. Don't include any explanations in your response."#, language, code);

        let response = self.inner.generate(GenerationRequest::new(model, prompt)).await;

        match response {
            Ok(generated) => Ok(generated.response),
            Err(e) => Err(anyhow::anyhow!("Error generating response: {}", e))
        }
    }

}


