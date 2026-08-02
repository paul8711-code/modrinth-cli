use clap::Parser;
use cli::Cli;

mod cli;

const USER_AGENT: &str = concat!(
    "paul8711-code/",
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_AUTHORS"),
    ")"
);

const API: &str = "https://api.modrinth.com";

fn main() {
    let cli = Cli::parse();
    println!("{}", USER_AGENT);
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .unwrap();
    let response = client
        .get(format!("{}/v2/search", API))
        // search for create mod and only return the first result
        .query(&[("query", "Create"), ("limit", "1")])
        .send()
        .unwrap()
        .text()
        .unwrap();

    println!("{:?}", response);
}
