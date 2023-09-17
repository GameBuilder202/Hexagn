pub fn get_line(src: &str, lineno: usize) -> String {
    let mut i = find_nth(src, &'\n', &(lineno - 1)) + 1;
    let mut res = String::new();

    while i < src.len() && src.chars().nth(i).unwrap() != '\n' {
        res += &src.chars().nth(i).unwrap().to_string();
        i += 1
    }

    res
}

pub fn find_nth(src: &str, c: &char, nth: &usize) -> usize {
    src.match_indices(*c).nth(*nth).unwrap().0
}

pub fn draw_arrows(start: usize, mut end: usize, lineno: usize) {
    let mut msg = format!("\x1b[31m{}", " ".repeat(lineno.to_string().len()));

    if end == start {
        end += 1
    }
    for _ in 0..=start {
        msg += " "
    }
    for _ in start..end {
        msg += "^"
    }

    eprintln!("{}\x1b[0m", msg)
}
