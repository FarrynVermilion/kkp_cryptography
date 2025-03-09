use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        for i in args.iter().skip(1) {
            println!("{}", i);
        }
    }
}