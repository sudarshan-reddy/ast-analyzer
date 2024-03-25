use analyzer::lsp::LspClient;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        // File to be documented
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        )
        // LSP address
        .arg(
            Arg::with_name("lsp")
                .short('l')
                .long("lsp")
                .value_name("LSP")
                .help("Sets the LSP address to use")
                .required(true)
                .takes_value(true),
        )
        // Get Line Number
        .arg(
            Arg::with_name("line")
                .short('n')
                .long("line")
                .value_name("LINE")
                .help("Sets the line number to use")
                .required(true)
                .takes_value(true),
        )
        // Get Column Number/Character
        .arg(
            Arg::with_name("col")
                .short('c')
                .long("col")
                .value_name("COL")
                .help("Sets the column number to use")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let dir = matches.value_of("dir").expect("dir name incorrect");
    let file_name = matches.value_of("file").expect("file name incorrect");
    let lsp = matches.value_of("lsp").expect("lsp address incorrect");
    let line = matches
        .value_of("line")
        .expect("line number incorrect")
        .parse::<u32>()
        .expect("line number incorrect");
    let col = matches
        .value_of("col")
        .expect("column number incorrect")
        .parse::<u32>()
        .expect("column number incorrect");
    let mut lsp_client = LspClient::new(lsp).await?;
    lsp_client.send_initialize_request(dir).await?;
    lsp_client.get_definition(file_name, line, col).await?;

    Ok(())
}
