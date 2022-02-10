use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    port: Option<u16>,
    #[clap(short, long)]
    mode: Option<String>,
    #[clap(short, long)]
    help: Option<String>,
}

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

pub fn start() {
    let args = Args::parse();
    if args.help.is_some() {
        usage();
    }

    let _port = args.port.unwrap_or(4000);
    match args.mode {
        Some(ref mode) => match mode.as_str() {
            "html" => println!("HTML selected"),
            "rest" => println!("REST selected"),
            _ => usage(),
        },
        None => println!("REST selected"),
    };
}
