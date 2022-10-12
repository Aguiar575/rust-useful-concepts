use std::time::{Duration, Instant};

fn main() {
    simple_for_loop_with_some_ownership_and_lambda();

    while_mutable_loop();

    loop_through_times();
}

fn loop_through_times() {
    let mut count: i32 = 0;
    //this represents 1 second
    let time_limit: Duration = Duration::new(1, 0);
    let start: Instant = Instant::now();
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}

fn while_mutable_loop() {
    let mut samples: Vec<&str> = vec![];
    while samples.len() < 10 {
        samples.push("value");
        println!("value pushed");
    }
}

fn simple_for_loop_with_some_ownership_and_lambda() {
    let container: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for item in &container {
        println!("{}", item);
    }
    //here container will dies because ownership
    container.into_iter().for_each(|item: i32| {
        println!("{}", item);
    });
}
