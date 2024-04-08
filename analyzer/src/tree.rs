use std::collections::HashMap;

use anyhow::Result;
use tracing::{info, trace};
/// tree.rs implements CodeWalker. The goal of this package is when given a method,
/// it will comb through the project and get all the defintions and references of that method.
use tree_sitter::{self, Node, Parser};
use walkdir::WalkDir;

/// CodeWalker holds context of the projet. It can take multiple backend implementations that could
/// range from in-memory to Tantivy/Lucene based implementations.
pub struct CodeWalker<I>
where
    I: Indexer,
{
    files: Vec<FileData>,
    indexer: I,
}

pub trait Indexer {
    fn index(&self, node: &Node, source_code: &[u8]);
}

struct FileData {
    file: String,
    language: String,
}

impl<I: Indexer> CodeWalker<I> {
    pub fn new_project(dir: &str, indexer: I) -> Self {
        let files = get_files_and_language(dir);
        CodeWalker { files, indexer }
    }

    /// This function should ideally be a background task that indexes the project.
    /// This way we should be able to be reasonably operational without waiting for
    /// synchronous indexing.
    pub async fn index_project(&mut self) -> Result<()> {
        let mut parsers = HashMap::new();
        for file in self.files.iter() {
            match file.language.as_str() {
                "go" => {
                    info!("Indexing file: {}, language: {}", file.file, file.language);
                    let parser = parsers.entry("go").or_insert_with(|| {
                        let mut parser = Parser::new();
                        parser.set_language(tree_sitter_go::language()).unwrap();
                        parser
                    });
                    let source_code = std::fs::read_to_string(&file.file)?;
                    let tree = parser
                        .parse(&source_code, None)
                        // TODO: Ideally this should not exit and continue with a warning.
                        .ok_or(anyhow::anyhow!("Error parsing code"))?;
                    let root_node = tree.root_node();
                    self.indexer.index(&root_node, &source_code.as_bytes());
                }
                _ => {
                    trace!("Unsupported language: {}", file.language);
                    continue;
                }
            }
        }

        Ok(())
    }
}

fn get_files_and_language(dir: &str) -> Vec<FileData> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let file = entry.path().to_str().unwrap().to_string();
            let language = get_language(&file);
            files.push(FileData { file, language });
        }
    }
    files
}

fn get_language(file: &str) -> String {
    let language = match file.split('.').last() {
        Some("go") => "go",
        _ => "unknown",
    };
    language.to_string()
}
