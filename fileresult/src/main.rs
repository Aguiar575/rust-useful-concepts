use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File 
        {
           name: String::from(name),
           data: Vec::new()
        }
    }
    
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut file: File = File::new(name);
        file.data = data.clone();
        file
    }

    fn read(
        self: &File,
         save_to: &mut Vec<u8>
    ) -> Result<usize, String> {
        let mut temp: Vec<u8> = self.data.clone();
        let read_length: usize = temp.len();
        save_to.reserve(read_length);
        save_to.append(&mut temp);
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg: String = String::from("Interrupted by signal!!");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg: String = String::from("Interrupted by signal!!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4: File = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = open(f4).unwrap();
    let f4_length: usize = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text: std::borrow::Cow<str> = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", f4.name, f4_length);
    println!("{}", text);
}
