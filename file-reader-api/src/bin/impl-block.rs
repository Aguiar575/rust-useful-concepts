#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main(){
    let file: File = File::new("file.txt");

    println!("{:?}", file);
    println!("{} is {} bytes long", &file.name, file.data.len());
}