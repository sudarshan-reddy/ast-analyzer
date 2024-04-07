use tree_sitter::Node;

use crate::tree::Indexer;

pub struct IndexData {
    pub value: String,
}

pub trait IndexStore {
    fn set(&mut self, key: &str, value: IndexData);
    fn get(&self, key: &str) -> String;
}

pub struct MethodIndexer {
    store: Box<dyn IndexStore>,
}

impl MethodIndexer {
    pub fn new(store: Box<dyn IndexStore>) -> Self {
        MethodIndexer { store }
    }

    // Helper function to extract the text for a given node
    fn get_node_text(&self, node: &Node, source_code: &[u8]) -> String {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let text = &source_code[start_byte..end_byte];
        String::from_utf8(text.to_vec()).expect("Found invalid UTF-8")
    }

    fn get_method_name(&self, node: &Node, source_code: &[u8]) -> Option<String> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "field_identifier" || child.kind() == "method_spec" {
                return Some(self.get_node_text(&child, source_code));
            }
        }
        None
    }
}

impl Indexer for MethodIndexer {
    fn index(&mut self, node: &Node, source_code: &[u8]) {
        if node.kind() == "method_declaration" {
            // Get the name of the method
            // TODO: Add more node types to extract
            // For method declaration nodes, print the full method including its body
            let text = self.get_node_text(node, source_code);
            let method_name = self.get_method_name(node, source_code);
            if let Some(method_name) = method_name {
                self.store.set(
                    &method_name.clone(),
                    IndexData {
                        value: text.clone(),
                    },
                );
            }
        }

        // Recursively traverse child nodes
        for child in node.children(&mut node.walk()) {
            self.index(&child, source_code);
        }
    }
}
