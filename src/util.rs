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

pub fn print_error(err: &str, src: &String, start: usize, end: usize, lineno: usize) {
    eprintln!("Error: {} at line {}", err, lineno);
    eprintln!("{}: {}", lineno, get_line(&src, lineno));
    draw_arrows(start, end, lineno);
}

#[macro_export]
macro_rules! unwrap_or_err {
    ($try:expr, $err:literal) => {{
        let res = $try;
        match res {
            Ok(_res) => _res,
            Err(_) => panic!("{:?}", $err),
        }
    }};

    ($try:expr, ($src:expr, $start:expr, $end:expr, $lineno:expr, $err:ident)) => {{
        let res = $try;
        match res {
            Some(_res) => _res,
            None => {
                print_error(&$err, &$src, $start, $end, $lineno);
                exit(1)
            }
        }
    }};
}
