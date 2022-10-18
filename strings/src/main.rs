fn simple_grep_with_enumeration_approach(quote: &str, search_term: &str) {
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{} : {}", line, line_num);
        }
    }
}

fn simple_grep_with_for_loop(quote: &str, search_term: &str) {
    let mut line_num: usize = 1;

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{} : {}", line, line_num);
        }
        line_num += 1;
    }
}

fn main() {
    let search_term: &str = "picture";
    let quote: &str = 
    "/ Every face, every shop, bedroom window, public house, and 
    dark saquare is a picture feverishly turned--in search of what? 
    Is is the same with books.
    What do wee seek through millions of pages?";

    simple_grep_with_for_loop(quote, search_term);

    simple_grep_with_enumeration_approach(quote, search_term);
}