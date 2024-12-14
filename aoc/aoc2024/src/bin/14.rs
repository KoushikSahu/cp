use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 12:09:10 PM(12:09:10) IST(+05:30) 14-12-2024 Sat
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let h = 103;
    let w = 101;
    let t = 100;
    let mut b = vec![vec![0i128; w as usize]; h as usize];
    while let Some(s) = sc.try_next() {
        let s1 = s.split_whitespace().collect::<Vec<&str>>();
        let p = s1[0][2..]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let v = s1[1][2..]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let x = (((p[1] + t * v[1]) % h) + h) % h;
        let y = (((p[0] + t * v[0]) % w) + w) % w;
        b[x as usize][y as usize] += 1;
    }
    let mut v = vec![vec![0i128; 2]; 2];
    for i in 0..h {
        for j in 0..w {
            if i == h / 2 || j == w / 2 {
                continue;
            }
            let x = if i < h / 2 { 0 } else { 1 };
            let y = if j < w / 2 { 0 } else { 1 };
            v[x][y] += b[i as usize][j as usize];
        }
    }
    let mut ans = 1i128;
    for i in 0..2 {
        for j in 0..2 {
            ans *= v[i][j];
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
