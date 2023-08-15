use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;
use std::cmp::min;

/*
    Author: Koushik Sahu
    Created: 12:07:54 AM(00:07:54) IST(+05:30) 14-08-2023 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n: i32 = sc.next();
    let mut a: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let s: i32 = sc.next();
        let v: Vec<i32> = (0..s).map(|_| sc.next()).collect();
        a.push(v);
    }
    for i in 0..n as usize {
        a[i].sort();
    }
    let mut mx1 = i32::MAX;
    let mut mx2 = i32::MAX;
    let mut sum: i64 = 0;
    for i in 0..n as usize {
        mx1 = min(mx1, a[i][0]);
        mx2 = min(mx2, a[i][1]);
        sum += a[i][1] as i64;
    }
    writeln!(wr, "{}", sum - mx2 as i64 + mx1 as i64).ok();
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
