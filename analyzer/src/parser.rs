use tree_sitter::{Parser, Node};
use anyhow::Result;
use std::sync::mpsc::{Sender, Receiver};

pub struct LangParser {
    pub language : Language,
    pub tree : tree_sitter::Tree,
    pub source_code: Vec<u8>,
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Copy, Clone)]
pub enum Language{ 
    Go
}

impl LangParser {
    // TODO: This should take the whole directory so all the symbols can be extracted
    pub fn new(language: Language, code: &str) ->  Result<LangParser> {
        let mut parser = LangParser::new_parser(language)?;
        let tree = parser.parse(code, None).ok_or(anyhow::anyhow!("Error parsing code"))?;
        let source_code = code.as_bytes();
        let (tx, rx) = std::sync::mpsc::channel();
        Ok(LangParser {
            language,
            tree,
            source_code: source_code.to_vec(),
            tx,
            rx,
        })
    }

    pub fn new_parser(language: Language) -> Result<Parser> {
        let mut parser = Parser::new();
        match language {
            Language::Go => parser.set_language(tree_sitter_go::language())?,
        };
        Ok(parser)
    }


    pub fn retrieve_all_methods(&self) -> Vec<String> {
        let root = self.tree.root_node();
        let mut methods = Vec::new();
        self.traverse(&root, &self.source_code);
        for message in self.rx.iter() {
            methods.push(message);
        }
        methods
    }

    fn traverse(&self, node: &Node, source_code: &[u8]) {
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
    fn get_node_text(&self, node: &Node, source_code: &[u8]) -> String {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let text = &source_code[start_byte..end_byte];

        String::from_utf8(text.to_vec()).expect("Found invalid UTF-8")
    }

}

