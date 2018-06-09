extern crate lab_common;

use lab_common::get_user_input;

fn part1() {
    let array: [usize; 4] = [1, 2, 3, 4];
    // println!("{}", array[4]);

    for x in &array {
        print!("{} ", x);
    }
}

// 123 % 10 = (3 / (10^0)) = 3
// 120 % 100 = (20 / 10^1) = 2
// 100 % 1000 = (100 / 10^2) = 1
fn sum_digits(input: isize) -> usize {
    let mut input = input.abs();

    let mut pow: u32 = 1;
    let mut sum: isize = 0;
    loop {
        let divisor = 10isize.pow(pow);

        let remainder = input % divisor;
        input -= remainder;
        sum += remainder / 10isize.pow(pow - 1);

        if input == 0 {
            break;
        }

        pow += 1;
    }
    assert!(sum >= 0);

    return sum as usize;
}

fn is_divisible_by_3(number: isize) -> bool {
    match digits_sum {
        0 | 1 | 2 | 4 | 5 | 7 | 8 => false,
        3 | 6 | 9 => true,
        _ => is_divisible_by_3(digits_sum as isize),
    }
}

/// Prompts the user for a positive integer and returns if it is divisible by three or not.
/// Returns the entered number as well as the divibility by 3 as a boolean.
fn check_divisibility() -> (isize, bool) {
    let input: isize = get_user_input(Some("Please enter a positive integer: "));
    let is_divisible: bool = is_divisible_by_3(input);
    (input, is_divisible)
}

fn part2() {
    let (input_number, is_divisible_by_3): (isize, bool) = check_divisibility();
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

#[test]
fn digits_summing() {
    let test_sum = |number: isize, sum: usize| {
        assert_eq!(sum_digits(number), sum);
    };

    test_sum(123, 6);
    test_sum(100, 1);
    test_sum(0, 0);
    test_sum(9999, 9+9+9+9);
    test_sum(54310, 5+4+3+1+0);
}

#[test]
fn divisibility_by_3() {
    let test_divisibility = |number: isize, is_divisible: bool| {
        assert_eq!(is_divisible, is_divisible_by_3(number));
    };

    test_divisibility(333, true);
    test_divisibility(334, false);
    test_divisibility(300, true);
    test_divisibility(2, false);
    test_divisibility(1, false);
    test_divisibility(3*1583, true);

    for i in 2..1000 {
        test_divisibility(3*i, true);
        test_divisibility((3*i) - 1, false);
        test_divisibility((3*i) + 1, false);
    }
}
