use std::collections::{HashMap, HashSet};
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 10:23:25 PM(22:23:25) IST(+05:30) 22-12-2024 Sun
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let MOD = 16777216;
    let itrs = 2000;
    let mut mp = vec![vec![vec![vec![0; 19]; 19]; 19]; 19];
    while let Some(s) = sc.try_next() {
        let mut prices = Vec::<i64>::new();
        let mut num = s.parse::<i64>().unwrap();
        for _ in 0..itrs {
            let n1 = num * 64;
            num = (n1 ^ num) % MOD;
            let n2 = num / 32;
            num = (n2 ^ num) % MOD;
            let n3 = num * 2048;
            num = (n3 ^ num) % MOD;
            prices.push(num);
        }
        for p in prices.iter_mut() {
            *p = *p % 10;
        }
        let mut diffs = Vec::<i64>::new();
        for i in 1..prices.len() {
            diffs.push(prices[i] - prices[i - 1]);
        }
        let mut seen = vec![vec![vec![vec![false; 19]; 19]; 19]; 19];
        for i in 4..diffs.len() + 1 {
            let key = Vec::<i8>::from(
                diffs[i - 4..i]
                    .iter()
                    .map(|&x| x as i8)
                    .collect::<Vec<i8>>(),
            );
            if mp[(key[0] + 9) as usize][(key[1] + 9) as usize][(key[2] + 9) as usize]
                [(key[3] + 9) as usize]
                == 0
            {
                mp[(key[0] + 9) as usize][(key[1] + 9) as usize][(key[2] + 9) as usize]
                    [(key[3] + 9) as usize] = prices[i];
                seen[(key[0] + 9) as usize][(key[1] + 9) as usize][(key[2] + 9) as usize]
                    [(key[3] + 9) as usize] = true;
            } else {
                if !seen[(key[0] + 9) as usize][(key[1] + 9) as usize][(key[2] + 9) as usize]
                    [(key[3] + 9) as usize]
                {
                    mp[(key[0] + 9) as usize][(key[1] + 9) as usize][(key[2] + 9) as usize]
                        [(key[3] + 9) as usize] += prices[i];
                    seen[(key[0] + 9) as usize][(key[1] + 9) as usize][(key[2] + 9) as usize]
                        [(key[3] + 9) as usize] = true;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..19 {
        for j in 0..19 {
            for k in 0..19 {
                for l in 0..19 {
                    ans = ans.max(mp[i][j][k][l]);
                }
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
    //let t: i32 = scan.next();
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
}
