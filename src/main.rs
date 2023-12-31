use clap::Parser;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

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

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn test_add() {
        assert_eq!(add(40, 2), 42);
    }
}
