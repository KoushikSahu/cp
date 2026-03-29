use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 05:49:58 PM(17:49:58) IST(+05:30) 28-03-2026 Sat
*/

#[derive(Debug)]
struct Num {
    x: i32,
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        other.x == self.x
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> Ordering {
        other.x.cmp(&self.x)
    }
}

impl Eq for Num {}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let q = sc.next::<i32>();
    let mut bh = BinaryHeap::new();
    for _ in 0..q {
        let (t, h) = (sc.next::<i32>(), sc.next::<i32>());
        match t {
            1 => {
                bh.push(Num { x: h });
            }
            2 => {
                while !bh.is_empty() && bh.peek().unwrap().x <= h {
                    dbg!(bh.peek().unwrap().x);
                    bh.pop();
                }
            }
            _ => (),
        }
        writeln!(wr, "{}", bh.len()).unwrap();
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
    //let t: i32 = scan.next();
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
}
