use std::cmp::min;
use std::collections::BinaryHeap;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 09:52:39 PM(21:52:39) IST(+05:30) 18-12-2024 Wed
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut pts = Vec::<Vec<i32>>::new();
    while let Some(s) = sc.try_next() {
        let pt = s
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        pts.push(pt);
    }
    let h = 71;
    let w = 71;
    let consider = 1024;
    let mut s = vec![vec!['.'; h]; w];
    for i in 0..min(consider, pts.len()) {
        s[pts[i][1] as usize][pts[i][0] as usize] = '#';
    }
    let start = (0, 0);
    let end = (h - 1, w - 1);
    let mut dist = vec![vec![std::i32::MAX; h]; w];
    let mut bh = BinaryHeap::<(i32, i32, i32)>::new();
    bh.push((0, start.0, start.1));
    dist[start.0 as usize][start.1 as usize] = 0;
    let valid = |x: i32, y: i32| x >= 0 && x < h as i32 && y >= 0 && y < w as i32;
    let dx = vec![0, 0, -1, 1];
    let dy = vec![-1, 1, 0, 0];
    while !bh.is_empty() {
        let pt = bh.pop().unwrap();
        for i in 0..4 {
            let x = pt.1 + dx[i];
            let y = pt.2 + dy[i];
            if valid(x, y)
                && s[x as usize][y as usize] != '#'
                && dist[x as usize][y as usize] > 1 + dist[pt.1 as usize][pt.2 as usize]
            {
                dist[x as usize][y as usize] = 1 + dist[pt.1 as usize][pt.2 as usize];
                bh.push((1 + dist[pt.1 as usize][pt.2 as usize], x, y));
            }
        }
    }
    writeln!(wr, "{}", dist[end.0][end.1]).unwrap();
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
