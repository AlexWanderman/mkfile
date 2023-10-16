use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return;
    }

    for arg in args[1..].iter() {
        match File::create(arg) {
            Ok(_) => println!("{arg}: Created"),
            Err(e) => println!("{arg}: {e}"),
        };
    }
}
