use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 01:30:01 AM(01:30:01) IST(+05:30) 16-07-2024 Tue
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut h: i32 = sc.next();
    let n: i32 = sc.next();
    let a: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    let c: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    let mut pq: BinaryHeap<(Reverse<i64>, i32, i32)> = BinaryHeap::new();
    for i in 0..n as usize {
        pq.push((Reverse(1 as i64), a[i], c[i]));
    }
    let mut ans: i64 = 1;
    while h > 0 {
        let curr = pq.pop();
        if let Some(curr_val) = curr {
            h -= curr_val.1;
            pq.push((Reverse(curr_val.0.0+curr_val.2 as i64), curr_val.1, curr_val.2));
            ans = ans.max(curr_val.0.0);
        }else{
            break;
        }
    }
    writeln!(wr, "{}", ans).unwrap();
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
