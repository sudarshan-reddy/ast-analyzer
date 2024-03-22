use tree_sitter::{Language, Parser, Node};
use anyhow::Result;
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
        if node.kind() == "method_declaration" {
            // TODO: Add more node types to extract
            // For method declaration nodes, print the full method including its body
            let text = self.get_node_text(node, source_code);
            self.tx.send(text).expect("Error sending method");
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

