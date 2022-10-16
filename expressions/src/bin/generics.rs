use std::ops::{Add};
use std::time::{Duration};

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats: f64 = add(1.2, 3.4);
    let ints: i32 = add(10, 20);
    let durations: Duration = add(
        Duration::new(5, 0),
        Duration::new(10, 0)
    );

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}