fn main() {
    different_representation_of_types();

    print_float();

    print_different_base_numbers();

    compare_numbers();
}

fn different_representation_of_types() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 21i32;
    let addition: i32 = twenty + twenty_one + twenty_two;

    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );
}

fn print_float() {
    let one_milion: i64 = 1_000_000;
    println!("{}", one_milion.pow(2));
    
    let forty_twos: [f64; 3] = [42.0, 42.0f64, 42.0_f64];
    println!("{:2}", forty_twos[0]);
}

fn print_different_base_numbers() {
    let three: i32 = 0b11;
    let thirty: i32 = 0o36;
    let three_hundred: i32 = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

fn compare_numbers() {
    let g: i32 = 2;
    let h: u16 = 3;

    if h > (g as u16) {
        print!("great!");
    }
}
