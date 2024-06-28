use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 12:52:09 AM(00:52:09) IST(+05:30) 28-06-2024 Fri
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n: i32 = sc.next();
    let k: i32 = sc.next();
    let mut a: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    if k > 0 {
        writeln!(wr, "Yes").ok();
        a.sort();
        for i in 0..n as usize {
            write!(wr, "{:} ", a[i]).ok();
        }
        writeln!(wr, "").ok();
    } else {
        a.sort_by(|x, y| y.cmp(x));
        let mut sm: i64 = 0;
        for i in &a {
            sm += *i as i64;
        }
        if sm < k as i64 {
            writeln!(wr, "No").ok();
        } else {
            writeln!(wr, "Yes").ok();
            for i in 0..n as usize {
                write!(wr, "{:} ", a[i]).ok();
            }
            writeln!(wr, "").ok();
        }
    }
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
    //let t: i32 = scan.next();
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
}
