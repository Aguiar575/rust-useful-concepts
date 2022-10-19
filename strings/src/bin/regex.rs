use regex::Regex;
use clap::{App, Arg};

fn main() {
    let quote: &str = 
    "/ Every face, every shop, bedroom window, public house, and 
    dark saquare is a picture feverishly turned--in search of what? 
    Is is the same with books.
    What do wee seek through millions of pages?";

    regex_grep(quote);
    cli_regex_method(quote);
}


//usage example cargo run --bin regex -- picture
fn cli_regex_method(quote: &str) {
    let args: clap::ArgMatches = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern: &str = args.value_of("pattern").unwrap();
    let re: Regex = Regex::new(pattern).unwrap();

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn regex_grep(quote: &str) {
    let re: Regex = Regex::new("picture").unwrap();
    for line in quote.lines() {
        let contains_substring: Option<regex::Match> = re.find(line);

        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}