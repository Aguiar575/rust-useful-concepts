fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 21i32;

    let addition: i32 = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_milion: i64 = 1_000_000;
    println!("{}", one_milion.pow(2));

    let forty_twos: [f64; 3] = [
        42.0,
        42.0f64,
        42.0_f64
    ];
    println!("{:2}", forty_twos[0]);
}
