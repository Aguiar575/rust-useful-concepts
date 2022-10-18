fn main() {
    let ctx_lines: usize = 2;
    let needle: &str = "oo";
    let haystack: &str = 
    "/ Every face, every shop, bedroom window, public house, and 
    dark saquare is a picture feverishly turned--in search of what? 
    Is is the same with books.
    What do wee seek through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(
                    usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound: usize = 
                tag.saturating_sub(ctx_lines);
            let upper_bound: usize = 
                tag + ctx_lines;

            if(i >= lower_bound) && (i <= upper_bound) {
                let line_as_string: String = String::from(line);
                let local_ctx: (usize, String) = (i, line_as_string);
                ctx[j].push(local_ctx);
            } 
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}