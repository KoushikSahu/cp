use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 10:26:34 PM(22:26:34) IST(+05:30) 02-12-2024 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut ans: i32 = 0;
    while let Some(report) = sc.try_nextreport::<i32>() {
        let mut final_valid = false;
        for to_skip in 0..report.len() + 1 {
            let mut r: Vec<i32> = Vec::new();
            for i in 0..report.len() {
                if i != to_skip {
                    r.push(report[i]);
                }
            }
            let mut valid = true;
            let mut last_diff: Option<i32> = None;
            for i in 1..r.len() {
                let curr_diff = r[i] - r[i - 1];
                let abs_diff = curr_diff.abs();
                if abs_diff >= 1 && abs_diff <= 3 {
                    if let Some(last_diff) = last_diff {
                        valid &= curr_diff * last_diff > 0;
                    }
                } else {
                    valid = false;
                }
                last_diff = Some(curr_diff);
            }
            final_valid |= valid;
        }
        if final_valid {
            ans += 1;
        }
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
    fn try_next<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok().expect("Failed parse"));
            }
            self.buf_str.clear();
            let bytes_read = self
                .reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            if bytes_read == 0 {
                return None; // End of file reached
            }
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
    fn try_nextreport<T: str::FromStr>(&mut self) -> Option<Vec<T>> {
        self.buf_str.clear();
        let bytes_read = self
            .reader
            .read_until(b'\n', &mut self.buf_str)
            .expect("Failed read");
        if bytes_read == 0 {
            return None;
        }
        return unsafe {
            let slice = str::from_utf8_unchecked(&self.buf_str);
            self.buf_iter = std::mem::transmute(slice.split_whitespace());
            Some(
                self.buf_iter
                    .clone()
                    .map(|x| x.parse().ok().expect("Failed parse"))
                    .collect(),
            )
        };
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
