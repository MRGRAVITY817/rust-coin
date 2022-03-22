mod blockchain;
mod constants;
mod db;
mod p2p;
mod rest;
mod result;
mod utils;
mod wallet;

use clap::{App, Arg};

fn usage() {
    println!(
        r#"
	Welcome to Rust Coin!\n\n
	Please use the following flags:\n
	-port:  Start the PORT of the server\n
	-mode:  Choose between 'html' and 'rest'\n
	"#
    );
}

#[tokio::main]
async fn main() {
    let matches = App::new("Rust Coin")
        .version("0.1")
        .author("Hoon Wee <coderoundhoon@gmail.com>")
        .about("A simple Proof-of-Work blockchain made of Rust")
        .arg(
            Arg::new("port")
                .short('p')
                .help("Sets a port number for connection")
                .default_value("4000"),
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .help("Starts the blockchain client with 'rest' or 'html' mode")
                .default_value("rest"),
        )
        .get_matches();

    match matches.value_of("port").unwrap_or("4000").parse::<u16>() {
        Ok(port) => match matches.value_of("mode") {
            Some(mode) => match mode {
                "html" => println!("HTML selected"),
                "rest" => rest::start(port).await,
                _ => usage(),
            },
            None => rest::start(port).await,
        },
        _ => usage(),
    }
}
