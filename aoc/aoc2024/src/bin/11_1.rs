use std::collections::HashMap;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;
use std::thread;

/*
    Author: Koushik Sahu
    Created: 01:35:30 PM(13:35:30) IST(+05:30) 11-12-2024 Wed
*/

fn digit_cnt(x: i128) -> i128 {
    if x == 0 {
        return 1;
    }
    let mut cnt = 0;
    let mut x = x;
    while x > 0 {
        cnt += 1;
        x /= 10;
    }
    cnt
}

fn digit_halves(x: &i128, dc: i128) -> (i128, i128) {
    let mut x = *x;
    let mut h1 = 0;
    let mut h2 = 0;
    let mut pos = 0;
    for _ in 0..dc / 2 {
        h1 += (x % 10) * 10i128.pow(pos);
        x /= 10;
        pos += 1;
    }
    let mut pos = 0;
    for _ in 0..dc / 2 {
        h2 += (x % 10) * 10i128.pow(pos);
        x /= 10;
        pos += 1;
    }
    (h2, h1)
}

fn dfs(x: &i128, n: i128, dp: &mut HashMap<(i128, i128), i128>) -> i128 {
    if n == 0 {
        return 1;
    }
    if dp.contains_key(&(*x, n)) {
        return dp[&(*x, n)];
    }
    let dc = digit_cnt(*x);
    let ans: i128;
    if *x == 0 {
        ans = dfs(&1, n - 1, dp);
    } else if dc % 2 == 0 {
        let dh = digit_halves(x, dc);
        ans = dfs(&dh.0, n - 1, dp) + dfs(&dh.1, n - 1, dp);
    } else {
        ans = dfs(&(*x * 2024), n - 1, dp);
    }
    dp.insert((*x, n), ans);
    return ans;
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let a = sc
        .try_next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();
    let mut ans: i128 = 0;
    let mut dp = HashMap::<(i128, i128), i128>::new();
    for x in a.iter() {
        ans += dfs(x, 75, &mut dp);
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
