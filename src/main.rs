use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    greeting: String,
    name: Option<String>,
    #[arg(long, default_value_t = false)]
    exclamation: bool,
}

fn main() {
    let args = Args::parse();
    println!("{} {}{}", args.greeting, args.name.unwrap_or(String::from("world")), if args.exclamation { "!" } else { "" });
}
