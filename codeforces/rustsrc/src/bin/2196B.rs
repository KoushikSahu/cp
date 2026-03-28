use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 09:18:44 PM(21:18:44) IST(+05:30) 24-03-2026 Tue
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<usize>();
    let a = (0..n).map(|_| sc.next::<i64>()).collect::<Vec<i64>>();
    let sqrt_n = (n as f64).sqrt() as i64;
    let valid = |x: i64| -> bool { x >= 0 && x < n as i64 };
    let mut ans = 0;
    for x in 0..=sqrt_n {
        for j in 0..n as i64 {
            let i = j - x * a[j as usize];
            if valid(i) {
                ans += if a[j as usize] * a[i as usize] == j - i {
                    1
                } else {
                    0
                };
            }
        }
    }
    for x in 0..=sqrt_n {
        for i in 0..n as i64 {
            if a[i as usize] <= sqrt_n {
                continue;
            }
            let j = i + a[i as usize] * x;
            if valid(j) {
                ans += if a[j as usize] * a[i as usize] == j - i {
                    1
                } else {
                    0
                };
            }
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
    let t: i32 = scan.next();
    for _ in 0..t {
        dbg!("new test");
        solve(&mut scan, &mut out);
    }
}
