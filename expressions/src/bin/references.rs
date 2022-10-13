fn main() {
    referencing_and_dereferencing();
    searching_for_needle();
}

fn referencing_and_dereferencing() {
    let a: i32 = 42;
    let r: &i32 = &a;
    let b: i32 = a + *r;
    println!("a + a = {}", b);
}

fn searching_for_needle() {
    let needle: i32 = 0o204;
    let haystack: [i32; 10] = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}