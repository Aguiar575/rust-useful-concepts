use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f: File = File::open("README.md").unwrap();
    let mut reader: BufReader<File> = BufReader::new(f);

    let mut line: String = String::new();

    loop{
        let len = reader.read_line(&mut line)
            .unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, line.len());
    }
}
