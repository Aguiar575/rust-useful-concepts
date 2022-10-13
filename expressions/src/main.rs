fn main() {
    normal_expression();
    martch_expression();
    break_return_value();
    match_multiple_values();
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn normal_expression() { 
    let n: i32 = 123456;
    let description: &str = if is_even(n)
    {
        "even"
    } else {
        "odd"
    };

    println!("{} is {}", n, description);
}

fn martch_expression() {
    let n: i32 = 654321;
    let description: &str = match is_even(n) {
        true => "even",
        false => "odd"
    };

    println!("{} is {}", n, description);
}

//break also returns value, this can be used to infine loops
fn break_return_value() {
    let n: i32 = loop {
        break 123;
    };

    println!("{}", n);
}

fn match_multiple_values() {
    let haystack: [i32; 10] = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _miss => "miss",
        };

        if result == "hit" {
            println!("{} : {}", item, result)
        }
    }
}