fn main() {
    fizzbuzz(100);
}

fn fizzbuzz(count: usize) {
    for i in 1..(count + 1) {
        match i {
            i if i % 5 == 0 => println!("FizzBuzz"),
            i if i % 3 == 0 => println!("Fizz"),
            _ => println!("{}", i),
        }
    }
}
