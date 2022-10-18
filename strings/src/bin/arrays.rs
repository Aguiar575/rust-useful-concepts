fn main() {
    let one: [u8; 3] = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank_one: [u8; 3] = [0; 3];
    let blank_two: [u8; 3] = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, blank_one, blank_two];

    for a in &arrays {
        print!("{:?}: ", a);

        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }

        println!("\t(Σ{:?} = {})", a, sum);
    }
}
