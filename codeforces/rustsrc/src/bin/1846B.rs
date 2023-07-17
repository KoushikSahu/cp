use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 10:20:03 PM(22:20:03) IST(+05:30) 17-07-2023 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let s: Vec<Vec<char>> = (0..3).map(|_| sc.next::<String>().chars().collect()).collect();
    let mut ans: String = String::from("DRAW");
    for i in 0..3 as usize {
        if s[i][0] != '.' && s[i][0] == s[i][1] && s[i][1] == s[i][2] {
            ans = s[i][0].to_string();
            break;
        }
        if s[0][i] != '.' && s[0][i] == s[1][i] && s[1][i] == s[2][i] {
            ans = s[0][i].to_string();
            break;
        }
    }
    if s[0][0] != '.' && s[0][0] == s[1][1] && s[1][1] == s[2][2] {
        ans = s[0][0].to_string();
    }
    if s[0][2] != '.' && s[0][2] == s[1][1] && s[1][1] == s[2][0] {
        ans = s[0][2].to_string();
    }
    writeln!(wr, "{}", ans).ok();
}

#[macro_export]
macro_rules! dbg{
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        eprintln!(
            concat!("{}:{}:{}: ",$(stringify!($a), " = {:?}, "),*),
            file!(), line!(), column!(), $($a),*
            );
        #[cfg(not(debug_assertions))]
        {};
    }
}

struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }
    fn next<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = BufWriter::new(stdout.lock());
    #[allow(unused_variables)]
    let t: i32 = 1;
    let t: i32 = scan.next(); 
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
}
