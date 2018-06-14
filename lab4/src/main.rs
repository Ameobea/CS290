extern crate lab_common;

use std::hint::unreachable_unchecked;

use lab_common::get_user_input;

fn get_name() -> String {
    get_user_input(
        Some("Enter your first and last name separate by a space: ")
    )
}

fn part1() {
    let mut name: String = get_name();
    while let None = name.find(' ') {
        println!("You must enter both a first and last name separated by a space!");
        name = get_name();
    }
    let space_index: usize = name.find(' ')
        .unwrap_or_else(|| unsafe { unreachable_unchecked() });

    let (first_name, last_name) = name.split_at(space_index);
    println!("{}, {}", last_name.trim_left(), first_name);
}

fn display_status(tuple: (isize, bool)) {
    let message: &'static str = match tuple {
        (i, true) if i >= 0 && i <= 10 => "Launch",
        (i, true) if i >= 11 && i <= 50 => "Standby",
        (i, false) if i >= 11 && i <= 50 => "Reboot",
        (i, false) if i >= 0 && i <= 10 => "Miss",
        _ => "Abort",
    };
    println!("{}", message);
}

fn part2() {
    let status: (isize, bool) = (9isize, false);
    display_status(status);
}

fn collatz(mut n: usize) -> usize {
    let mut count = 0;
    loop {
        match n {
            1 => { break; },
            nn if nn % 2 == 1 => { n = 3 * n + 1 },
            nn if nn % 2 == 0 => { n /= 2 },
            _ => unsafe { unreachable_unchecked() },
        }
        count += 1;
    }

    count
}

fn part3() {
    let input: usize = get_user_input(
        Some("Enter a positive integer with which to test the Collatz Conjecture: ")
    );
    let count = collatz(input);
    println!("Iterations: {}", count);
}

fn main() {
    part1();
    part2();
    part3();
}
