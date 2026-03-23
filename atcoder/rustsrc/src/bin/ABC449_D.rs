use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::ops::{Mul};
use std::str;

/*
    Author: Koushik Sahu
    Created: 06:21:47 PM(18:21:47) IST(+05:30) 14-03-2026 Sat
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let (l, r, d, u) = (sc.next::<i32>(), sc.next::<i32>(), sc.next::<i32>(), sc.next::<i32>());
    let mut ans = 0i64;
    for i in l..=r {
        if ((i % 2) + 2) % 2 == 0 {
            let uu = i.abs();
            let dd = i.abs().mul(-1);
            if uu < d || dd > u {continue;}
            ans += uu.min(u) as i64 - dd.max(d) as i64 + 1;
        }
    }
    for i in d..=u {
        if i != 0 && ((i % 2) + 2) % 2 == 0 {
            let ll = i.abs().mul(-1);
            let rr = i.abs();
            if ll > r || rr < l {continue;}
            ans += rr.min(r) as i64 - ll.max(l) as i64 + 1;
            if rr.min(r) == rr {ans -= 1;}
            if ll.max(l) == ll {ans -= 1;}
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
