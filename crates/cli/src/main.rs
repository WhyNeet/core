use args::CliArgs;
use clap::Parser;
use engine::parsing;

mod args;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let document = reqwest::get(args.url).await.unwrap().text().await.unwrap();

    let texts = parsing::parse_document(&document);

    for text in texts {
        println!("{text}");
    }
}
