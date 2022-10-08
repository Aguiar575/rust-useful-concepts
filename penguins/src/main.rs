use std::str::Lines;

fn main() {
    let penguin_data: &str = "\
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    fiordland penguin, 65
    invalid, data
    ";

    let records: Lines  = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.
            split(",")
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name: &str = fields[0];

        if let Ok(length) = fields[1].parse::<f32>(){
            println!("{}, {}cm", name, length);
        }
    }
}
