use std::cmp::min;
use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:08:06 PM(20:08:06) IST(+05:30) 23-02-2026 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<usize>();
    let (mut ax, mut ay, bx, by) = (sc.next::<usize>(), sc.next::<usize>(), sc.next::<usize>(), sc.next::<usize>());
    let mut p = vec![];
    for _ in 0..n {
        let x = sc.next::<usize>();
        p.push(vec![x, 0]);
    }
    for i in 0..n {
        let y = sc.next::<usize>();
        p[i][1] = y;
    }
    p.sort();
    let mut p_ends: Vec<Vec<usize>> = vec![];
    p_ends.push(vec![ax, ay, ay]);
    for i in 0..n {
        let b = p_ends.last_mut();
        match b {
            None => {
                p_ends.push(vec![p[i][0], p[i][1], p[i][1]]);
            },
            Some(b) => {
                if b[0] == p[i][0] {
                    b[1] = b[1].min(p[i][1]);
                    b[2] = b[2].max(p[i][1]);
                } else {
                    p_ends.push(vec![p[i][0], p[i][1], p[i][1]]);
                }
            }
        }
    }
    p_ends.push(vec![bx, by, by]);
    let mut dp = vec![vec![0; 2]; p_ends.len()];
    dp[0][0] = 0;
    dp[0][1] = 0;
    for i in 1..dp.len() {
        dp[i][0] = min(dp[i - 1][0] + p_ends[i - 1][1].abs_diff(p_ends[i - 1][2]) + p_ends[i - 1][0].abs_diff(p_ends[i][0]) + p_ends[i - 1][2].abs_diff(p_ends[i][1]),
            dp[i - 1][1] + p_ends[i - 1][2].abs_diff(p_ends[i - 1][1]) + p_ends[i - 1][0].abs_diff(p_ends[i][0]) + p_ends[i - 1][1].abs_diff(p_ends[i][1]));
        dp[i][1] = min(dp[i - 1][0] + p_ends[i - 1][1].abs_diff(p_ends[i - 1][2]) + p_ends[i - 1][0].abs_diff(p_ends[i][0]) + p_ends[i - 1][2].abs_diff(p_ends[i][2]),
            dp[i - 1][1] + p_ends[i - 1][2].abs_diff(p_ends[i - 1][1]) + p_ends[i - 1][0].abs_diff(p_ends[i][0]) + p_ends[i - 1][1].abs_diff(p_ends[i][2]));
    }
    writeln!(wr, "{}", min(dp[dp.len() - 1][0], dp[dp.len() - 1][1])).unwrap();
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
