use std::cmp::min;
use std::collections::BinaryHeap;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 09:52:39 PM(21:52:39) IST(+05:30) 18-12-2024 Wed
*/

fn valid(x: i32, y: i32, h: i32, w: i32) -> bool {
    x >= 0 && x < h && y >= 0 && y < w
}

fn dfs(
    pt: (i32, i32),
    s: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    end: (i32, i32),
    reachable: &mut bool,
) {
    if *reachable == true {
        return;
    }
    seen[pt.0 as usize][pt.1 as usize] = true;
    if (pt.0, pt.1) == end {
        *reachable = true;
    }
    let h = s.len() as i32;
    let w = s[0].len() as i32;
    let dx = vec![0, 0, -1, 1];
    let dy = vec![-1, 1, 0, 0];
    for i in 0..4 {
        let x = pt.0 + dx[i];
        let y = pt.1 + dy[i];
        if valid(x, y, h, w) && !seen[x as usize][y as usize] && s[x as usize][y as usize] != '#' {
            dfs((x, y), s, seen, end, reachable);
        }
    }
}

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
    for i in 0..pts.len() {
        s[pts[i][1] as usize][pts[i][0] as usize] = '#';

        if i >= consider {
            let start = (0, 0);
            let end = (h - 1, w - 1);

            let mut seen = vec![vec![false; w]; h];
            let mut reachable = false;
            dfs(
                start,
                &s,
                &mut seen,
                (end.0 as i32, end.1 as i32),
                &mut reachable,
            );

            if reachable == false {
                writeln!(wr, "{},{}", pts[i][0], pts[i][1]).unwrap();
                break;
            }
        }
    }
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
