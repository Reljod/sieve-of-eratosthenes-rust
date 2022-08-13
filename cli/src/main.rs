use clap::Parser;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    command: String,
}
