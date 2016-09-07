extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("--------------guessing number game----------------");
    let secret_number = rand::thread_rng().gen_range(1, 101);



}
