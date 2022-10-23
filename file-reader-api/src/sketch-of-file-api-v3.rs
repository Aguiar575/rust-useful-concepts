#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(
        name: &str,
        data: &Vec<u8>,
    ) -> File {
        File {
            name: String::from(name),
            data: data.clone()
        }
    }

    fn read(
        self: &File,
        save_to: &mut Vec<u8>,
    ) -> usize {
        let mut temp = self.data.clone();
        let read_length = temp.len();

        save_to.reserve(read_length);
        save_to.append(&mut temp);

        read_length
    }
}

fn open(file: &mut File) -> bool{
    true
}

fn close(file: &mut File) -> bool{
    true
}

fn main() {
    let mut file = File::new_with_data(
        "file.txt", 
        vec![114, 117, 115, 115, 33]);

    let mut buffer: Vec<u8> = vec![];

    open(&mut file);
    let file_length = read(&file, &mut buffer);
    close(&mut file);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", &file.name, file_length);
    println!("{}", text);
}