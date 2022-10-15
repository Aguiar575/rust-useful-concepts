#[derive(Debug, Clone, Copy)]
struct SomeStruct<'a> {
    num: &'a i32,
}

fn print_some_struct(the_struct: &SomeStruct) {
    println!("{:?}", the_struct);
}

fn biggest<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct<'a> {
    if a.num > b.num {
        a
    } else {
        b
    }
}

fn wrapping_all_borrowing_and_lifetime_together() {
    let some_struct_number: i32 = 3;
    let some_struct: SomeStruct = SomeStruct {
        num: &some_struct_number,
    };
    let bigger_struct: SomeStruct = SomeStruct { num: &5 };
    let bigger: &SomeStruct = biggest(&some_struct, &bigger_struct);
    print_some_struct(&bigger);
}

fn add_with_lifetimes<'a>(a: &'a i32, b: &'a i32) -> i32{
    *a + *b
}

fn action_of_sum_with_lifetimes() {
    let first_number: i32 = 3;
    let second_number: i32 = 5;
    let result: i32 = add_with_lifetimes(&first_number, &second_number);
    println!("{}", result);
}

fn main() {
    wrapping_all_borrowing_and_lifetime_together();
    action_of_sum_with_lifetimes();
}
