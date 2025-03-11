use std::io::stdout;

use args::CliArgs;
use clap::Parser;
use engine::parsing;
use renderer::CliTreeRenderer;

mod args;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let document = reqwest::get(args.url).await.unwrap().text().await.unwrap();

    let mut parser = parsing::DocumentParser::new(&document);
    let tree = parser.parse_document();

    println!("{tree:?}");

    let mut output_writer = stdout();
    for node in tree {
        CliTreeRenderer::render_tree(node, &mut output_writer)?;
    }

    Ok(())
}
