extern crate lazy_static;
extern crate rand;
extern crate regex;

mod dice;
use dice::Dicepool;
use std::env::args;

fn main() {
    let description = args().nth(1).unwrap_or(String::from("1d20"));
    match Dicepool::from_description(&description) {
        Ok(dicepool) => {
            println!("{:?}", dicepool);
            println!("{:?}", dicepool.roll());
        }
        Err(e) => println!("{:?}", e),
    }
}
