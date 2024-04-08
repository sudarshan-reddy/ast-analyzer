use std::sync::{Arc, Mutex};

use tree_sitter::Node;

use crate::tree::Indexer;

#[derive(Clone)]
pub struct IndexData {
    pub value: String,
}

pub trait IndexStore {
    fn set(&mut self, key: &str, value: IndexData);
    fn get(&self, key: &str) -> String;
    fn get_all(&self) -> std::collections::HashMap<String, String>;
}

pub struct MethodIndexer<S: IndexStore + Clone> {
    store: Arc<Mutex<S>>,
}

impl<S: IndexStore + Clone + 'static> MethodIndexer<S> {
    pub fn new(store: Arc<Mutex<S>>) -> Self {
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

impl<S: IndexStore + Clone + 'static> Indexer for MethodIndexer<S> {
    fn index(&self, node: &Node, source_code: &[u8]) {
        if node.kind() == "method_declaration" {
            // Get the name of the method
            // TODO: Add more node types to extract
            // For method declaration nodes, print the full method including its body
            let text = self.get_node_text(node, source_code);
            let method_name = self.get_method_name(node, source_code);
            if let Some(method_name) = method_name {
                let mut store = self.store.lock().unwrap();
                store.set(
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
