//Using the newtype pattern can strengthen a program by 
//preventing data from being silently used in inappropriate contexts.
struct Hostname(String);

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn main() {
    let ordinary_string: String = String::from("localhost");
    let host: Hostname = Hostname(ordinary_string.clone());

    connect(host);

    //this bellow will trigger 
    //connect(ordinary_string);
    //error[E0308]: mismatched types
    //--> ch3-newtype-pattern.rs:11:13
    //|
    //|     connect(ordinary_string);
    //|             ^^^^^^^^^^^^^^^ expected struct `Hostname`,
    //                             found struct `String`
}