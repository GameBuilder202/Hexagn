mod compiler;
pub use compiler::*;
pub mod lexer;

#[macro_export]
macro_rules! print_error
{
	($err:expr, $buf:ident, $src:expr, $start:expr, $end:expr, $lineno:expr) => {
		println_err!($err, $lineno);
		println_err!("{}: {}", $lineno, get_line(&$src, &$lineno));
		draw_arrows(&$start, &$end, &$lineno);
	}
}

#[macro_export]
macro_rules! unwrap_or_err
{
	($try:expr, $err:literal) => {
		{
			let res = $try;
			match res {
				Ok(_res) => _res,
				Err(_) => panic!("{:?}", $err)
			}
		}
	};

	($try:expr, ($buf:ident, $src:expr, $start:expr, $end:expr, $lineno:expr, $err:ident)) => {
		{
			let res = $try;
			match res {
				Some(_res) => _res,
				None => {
					println_err!("{} {}", $err, $lineno);
					println_err!("{}: {}", $lineno, get_line(&$src, &$lineno));
					draw_arrows(&$start, &$end, &$lineno);
					exit(1)
				}
			}
		}
	};
}
