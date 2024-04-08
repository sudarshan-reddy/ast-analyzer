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

pub struct GoIndexer<S: IndexStore + Clone> {
    store: Arc<Mutex<S>>,
}

impl<S: IndexStore + Clone + 'static> GoIndexer<S> {
    pub fn new(store: Arc<Mutex<S>>) -> Self {
        GoIndexer { store }
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

    fn get_struct_name(&self, node: &Node, source_code: &[u8]) -> Option<String> {
        for child in node.children(&mut node.walk()) {
            if child.kind() == "parameter_list" {
                return Some(self.get_node_text(&child, source_code));
            }
        }
        None
    }

    fn get_package_name(&self, node: &Node, source_code: &[u8]) -> Option<String> {
        // Direct match at the current node level
        if node.kind() == "package_clause" {
            let package_def = self.get_node_text(node, source_code);
            let package_name = package_def
                .strip_prefix("package ")
                .map(|s| s.trim().to_string());
            return package_name;
        }

        // Recursively search in child nodes
        let mut walker = node.walk();
        for child in node.children(&mut walker) {
            if let Some(name) = self.get_package_name(&child, source_code) {
                return Some(name);
            }
        }

        None // Return None if no package_clause node is found in any children
    }

    fn construct_key(
        &self,
        package_name: &str,
        struct_name: Option<&str>,
        method_name: &str,
    ) -> String {
        match struct_name {
            Some(struct_name) => format!("{}::{}::{}", package_name, struct_name, method_name),
            None => format!("{}::{}", package_name, method_name),
        }
    }

    fn traverse(&self, key_prefix: &str, node: &Node, source_code: &[u8]) {
        if node.kind() == "method_declaration" {
            // Get the name of the method
            // TODO: Add more node types to extract
            // For method declaration nodes, print the full method including its body
            let text = self.get_node_text(node, source_code);
            let method_name = self.get_method_name(node, source_code);
            let struct_name = self.get_struct_name(node, source_code);
            if let Some(method_name) = method_name {
                let key = self.construct_key(
                    key_prefix,
                    struct_name.as_ref().map(|s| s.as_str()),
                    &method_name,
                );
                let mut store = self.store.lock().unwrap();
                store.set(
                    &key,
                    IndexData {
                        value: text.clone(),
                    },
                );
            }
        }

        // Recursively traverse child nodes
        for child in node.children(&mut node.walk()) {
            self.traverse(key_prefix, &child, source_code);
        }
    }
}

impl<S: IndexStore + Clone + 'static> Indexer for GoIndexer<S> {
    fn index(&self, node: &Node, source_code: &[u8]) {
        let package_name = self
            .get_package_name(node, source_code)
            .unwrap_or("".to_string());
        self.traverse(&package_name, node, source_code);
    }
}
