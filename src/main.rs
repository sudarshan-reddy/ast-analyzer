use clap::{App, Arg};
use std::fs;

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("File Reader")
        .version("1.0")
        .author("Sudarsan Reddy <sudar.theone@gmail.com>")
        .about("Reads a file and prints its contents")
        .arg(Arg::with_name("file")
             .short('f')
             .long("file")
             .value_name("FILE")
             .help("Sets the input file to use")
             .required(true)
             .takes_value(true))
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut parser = lib::LangParser::new().unwrap();
    let tree = parser.parse(&contents).expect("Error parsing code");
    parser.traverse(&tree.root_node(), contents.as_bytes());

    let client = lib::LLMClient::new();
    for message in parser.rx.iter() {
        let result = client.get("golang", &message).await;
        println!("{}", result.unwrap());
    }

    Ok(())
}
