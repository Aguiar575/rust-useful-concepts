#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let file: File = File {
        name: String::from("file.txt"),
        data: Vec::new(),
    };

    println!("{:?}", &file);
    println!("{} is {} bytes long", file.name, file.data.len());
}