extern crate lazy_static;
extern crate rand;
extern crate regex;

mod dice;
use dice::Dicepool;

fn main() {
    match Dicepool::from_description("3d20") {
        Ok(dicepool) => {
            println!("{:?}", dicepool);
            println!("{:?}", dicepool.roll());
        }
        Err(e) => println!("{:?}", e),
    }
}
