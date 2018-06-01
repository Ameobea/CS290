extern crate lab_common;

use lab_common::get_user_input;

fn part1() {
    let array: [usize; 4] = [1, 2, 3, 4];
    // println!("{}", array[4]);

    for x in &array {
        print!("{} ", x);
    }
}

fn is_divisible_by_3(number: &str) -> Result<bool, String> {
    let mut is_err = false;
    let sum: usize = number.chars().fold(0usize, |acc: usize, digit_char: char| -> usize {
        if is_err {
            return 0;
        } else if digit_char == '\n' {
            return acc;
        }

        let parsed_char: u32 = match digit_char.to_digit(10) {
            Some(digit) => digit,
            None => {
                is_err = true;
                return 0;
            },
        };

        acc + (parsed_char as usize)
    });

    if is_err {
        Err("An invalid character was included in the provided number!  Please enter another.".into())
    } else {
        Ok(sum % 3 == 0)
    }
}

/// Prompts the user for a positive integer and returns if it is divisible by three or not.
/// Returns the entered number as well as the divibility by 3 as a boolean.
fn check_divisibility() -> (String, bool) {
    let input: String = get_user_input(Some("Please enter a positive integer: "));
    match is_divisible_by_3(&input) {
        Ok(divisible) => (input.trim().into(), divisible),
        Err(err_msg) => {
            println!("{}", err_msg);
            check_divisibility()
        }
    }
}

fn part2() {
    let (input_number, is_divisible_by_3): (String, bool) = check_divisibility();
    println!(
        "{} is {}divisible by 3.",
        input_number,
        if is_divisible_by_3 { "" } else { "not " }
    );
}

#[derive(Debug)]
struct Point {
    pub x: isize,
    pub y: isize,
}

fn part3() {
    let point = Point { x: 12, y: 3 };
    println!("{:?}", point);
}

fn main() {
    part1();
    println!("");
    part2();
    part3();
}
