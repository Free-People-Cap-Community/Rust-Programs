extern crate rand;
use rand::{thread_rng, Rng};
fn main() {
    let mut rnd = thread_rng();
    let b: bool = rnd.gen();
    println!("{}", b);
}
