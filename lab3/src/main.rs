extern crate lab_common;
extern crate rand;

use std::process::exit;

use rand::Rng;

use lab_common::get_user_input;

const MAX_NUMBER: usize = 1000;
const GUESSES: usize = 10;

fn main() {
    let answer: usize = rand::thread_rng().gen_range(0, MAX_NUMBER);
    println!("Guess a number from 0 to {}.", MAX_NUMBER);

    for i in 0..GUESSES {
        let guess: usize = get_user_input(Some("Guess: "));
        match guess {
            guess if guess == answer => {
                println!("You win!  It took you {} guesses.", i+1);
                exit(0);
            },
            guess if guess < answer => println!("Higher"),
            guess if guess > answer => println!("Lower"),
            _ => unreachable!(),
        }
    }

    println!("You lose!  The correct answer was {}.", answer);
}
