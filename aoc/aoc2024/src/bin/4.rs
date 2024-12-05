use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:18:09 PM(20:18:09) IST(+05:30) 04-12-2024 Wed
*/

fn valid(pts: &Vec<(i32, i32)>, n: &usize, m: &usize) -> bool {
    for pt in pts.iter() {
        if pt.0 < 0 || pt.0 >= *n as i32 || pt.1 < 0 || pt.1 >= *m as i32 {
            return false;
        }
    }
    true
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut inp: Vec<Vec<char>> = Vec::new();
    while let Some(s) = sc.try_next::<String>() {
        let chrs = s.chars().collect();
        inp.push(chrs);
    }
    let n = inp.len();
    let m = inp[0].len();
    let r: Vec<Vec<i32>> = vec![
        vec![0, 0, 0],
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![0, 0, 0],
        vec![-1, -2, -3],
        vec![-1, -2, -3],
        vec![-1, -2, -3],
    ];
    let c: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![0, 0, 0],
        vec![-1, -2, -3],
        vec![-1, -2, -3],
        vec![-1, -2, -3],
        vec![0, 0, 0],
        vec![1, 2, 3],
    ];
    let mut ans: i32 = 0;
    for i in 0..n as i32 {
        for j in 0..m as i32 {
            if inp[i as usize][j as usize] == 'X' {
                for k in 0..8 as usize {
                    let mut pts: Vec<(i32, i32)> = Vec::new();
                    for l in 0..3 as usize {
                        pts.push((i + r[k][l], j + c[k][l]));
                    }
                    if valid(&pts, &n, &m) {
                        let mut chrs: Vec<char> = Vec::new();
                        for pt in pts.iter() {
                            chrs.push(inp[pt.0 as usize][pt.1 as usize]);
                        }
                        if chrs.len() == 3 && chrs[0] == 'M' && chrs[1] == 'A' && chrs[2] == 'S' {
                            ans += 1;
                        }
                    }
                }
            }
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
