use std::cmp::max;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 05:37:28 PM(17:37:28) IST(+05:30) 07-12-2024 Sat
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let h = sc.next::<i32>();
    let w = sc.next::<i32>();
    let d = sc.next::<i32>();
    let mut s = Vec::<Vec<char>>::new();
    for _ in 0..h {
        let ss = sc.next::<String>().chars().collect::<Vec<char>>();
        s.push(ss);
    }
    let mut fs = Vec::<(i32, i32)>::new();
    for i in 0..h {
        for j in 0..w {
            if s[i as usize][j as usize] == '.' {
                fs.push((i, j));
            }
        }
    }
    let mut ans: i32 = 0;
    let manhattan =
        |(x1, y1): (i32, i32), (x2, y2): (i32, i32)| -> i32 { (x1 - x2).abs() + (y1 - y2).abs() };
    for i in 1..fs.len() {
        for j in 0..i {
            let mut h = 0;
            for k in 0..fs.len() {
                if i != k || j != k {
                    if manhattan(fs[i], fs[k]) <= d || manhattan(fs[j], fs[k]) <= d {
                        h += 1;
                    }
                }
            }
            ans = max(ans, h);
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
            return unsafe { Some(str::from_utf8_unchecked(&self.buf_str).to_owned()) };
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
