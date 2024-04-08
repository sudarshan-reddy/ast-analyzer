use std::sync::{Arc, Mutex};

use analyzer::indexer::IndexStore;
use analyzer::{indexer::GoIndexer, memory::InMemoryIndexStore, tree::CodeWalker};
use clap::{App, Arg};
use tracing::{debug, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let matches = App::new("Docthis")
        .version("1.0")
        .author("Sudarsan Reddy <sudar.theone@gmail.com>")
        .about("Helps Document Code")
        // Directory of the project
        .arg(
            Arg::with_name("dir")
                .short('d')
                .long("dir")
                .value_name("DIR")
                .help("Sets the directory of the project")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let dir = matches.value_of("dir").expect("dir name incorrect");

    let storage = Arc::new(Mutex::new(InMemoryIndexStore::new()));
    let go_indexer = GoIndexer::new(storage.clone());
    CodeWalker::new_project(dir, go_indexer)
        .index_project()
        .await?;

    let store = storage.lock().unwrap();
    let all_methods = store.get_all();
    for (k, v) in all_methods.iter() {
        debug!("Method: {}, Value: {}", k, v);
    }

    Ok(())
}
