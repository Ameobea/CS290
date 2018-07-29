use std::f64::consts::PI;

fn main() {
    part1();
    println!("----------");
    part2();
}

fn part1() {
    let circle = Circle {
        x: 10.,
        y: 10.,
        radius: 10.,
    };
    println!("Circle: {:?}\nArea: {}", circle, circle.area());

    let right_triangle = RTriangle {
        leg_1: 10.,
        leg_2: 5.,
    };
    println!(
        "Right Triangle: {:?}\nArea: {}",
        right_triangle,
        right_triangle.area()
    );
}

fn part2() {
    fn print_area<T: HasArea>(shape: &T) {
        println!("Area: {:?}", shape.area());
    }

    let shape = Circle {
        x: 10.,
        y: 10.,
        radius: 0.5,
    };
    print_area(&shape);
}

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[derive(Debug)]
struct RTriangle {
    leg_1: f64,
    leg_2: f64,
}

impl HasArea for RTriangle {
    fn area(&self) -> f64 {
        0.5 * self.leg_1 * self.leg_2
    }
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}
