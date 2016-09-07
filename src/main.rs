extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("--------------guessing number game----------------");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("\n");
        println!("----------please input your guess-------------");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("failed to read line.");

        let guess: u32 = guess.trim().parse()
            .expect("please type a number.");

        println!("you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("too small."),
            Ordering::Greater => println!("too big."),
            Ordering::Equal   => {
                println!("you win!");
                break;
            }
        }
    }
    println!("the secret_number is indeed {}", secret_number);
}
