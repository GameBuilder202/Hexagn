use std::io::Write;

pub fn get_line(src: &String, lineno: usize) -> String {
    let mut i = find_nth(src, &'\n', &(lineno - 1)) + 1;
    let mut res = String::new();

    while i < src.len() && src.chars().nth(i).unwrap() != '\n' {
        res += &src.chars().nth(i).unwrap().to_string();
        i += 1
    }

    res
}

pub fn find_nth(src: &String, c: &char, nth: &usize) -> usize {
    src.match_indices(*c).nth(*nth).unwrap().0
}

pub fn draw_arrows(start: usize, end: usize, lineno: usize) {
    let start = start + lineno.to_string().len() + 2;
    let end = end + lineno.to_string().len() + 2;

    std::io::stderr()
        .write_fmt(format_args!("\x1b[31m"))
        .unwrap();
    for _ in 0..start {
        std::io::stderr().write_fmt(format_args!(" ")).unwrap()
    }
    for _ in start..end {
        std::io::stderr().write_fmt(format_args!("^")).unwrap()
    }
    std::io::stderr()
        .write_fmt(format_args!("\x1b[0m\n"))
        .unwrap();
}
