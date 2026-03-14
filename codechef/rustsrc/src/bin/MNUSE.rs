use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:19:44 PM(20:19:44) IST(+05:30) 11-03-2026 Wed
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let (n, k, s, m) = (sc.next::<i64>(), sc.next::<i64>(), sc.next::<i64>(), sc.next::<i64>());
    if m > 1 && m < n {
        for i in 0..=k {
            let (low, high) = (i + (k - i) * (m + 1), i * (m - 1) + (k - i) * n);
            if s >= low && s <= high {
                writeln!(wr, "{}", 0).unwrap();
                return;
            }
        }
        writeln!(wr, "{}", 1).unwrap();
        return;
    } else if m == 1 {
        for i in 0..=k {
            let (low, high) = (2 * (k - i) + i, n * (k - i) + i);
            if s >= low && s <= high {
                writeln!(wr, "{}",  i).unwrap();
                return;
            }
        }
    } else { 
        for i in 0..=k {
            let (low, high) = (k - i + n * i, (n - 1) * (k - i) + n * i);
            if s >= low && s <= high {
                writeln!(wr, "{}",  i).unwrap();
                return;
            }
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

#[allow(dead_code)]
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
    fn try_next(&mut self) -> Option<String> {
        loop {
            self.buf_str.clear();
            let bytes_read = self
                .reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            if bytes_read == 0 {
                return None; // End of file reached
            }
            return unsafe { Some(str::from_utf8_unchecked(&self.buf_str).trim().to_owned()) };
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
