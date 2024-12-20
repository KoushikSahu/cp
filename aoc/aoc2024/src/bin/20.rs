use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 10:08:45 PM(22:08:45) IST(+05:30) 20-12-2024 Fri
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut f = Vec::<Vec<char>>::new();
    while let Some(s) = sc.try_next() {
        f.push(s.chars().collect());
    }
    let h = f.len();
    let w = f[0].len();
    let MAX = 1e9 as i32 + 5;
    let mut dist = vec![vec![MAX; w]; h];
    let mut start = (-1, -1);
    let mut end = (-1, -1);
    for i in 0..h {
        for j in 0..w {
            if f[i][j] == 'S' {
                start = (i as i32, j as i32);
                f[i][j] = '.';
            } else if f[i][j] == 'E' {
                end = (i as i32, j as i32);
                f[i][j] = '.';
            }
        }
    }
    let mut curr = start;
    let dx = [0, 0, -1, 1];
    let dy = [1, -1, 0, 0];
    let valid = |x: i32, y: i32| -> bool { x >= 0 && x < h as i32 && y >= 0 && y < w as i32 };
    dist[curr.0 as usize][curr.1 as usize] = 0;
    while curr != end {
        for i in 0..4 {
            let pt = (curr.0 + dx[i], curr.1 + dy[i]);
            if valid(pt.0, pt.1)
                && f[pt.0 as usize][pt.1 as usize] != '#'
                && dist[pt.0 as usize][pt.1 as usize] == MAX
            {
                dist[pt.0 as usize][pt.1 as usize] = dist[curr.0 as usize][curr.1 as usize] + 1;
                curr = pt;
                break;
            }
        }
    }
    let et = dist[end.0 as usize][end.1 as usize];
    let mut edist = dist.clone();
    for i in 0..h {
        for j in 0..w {
            if edist[i][j] < MAX {
                edist[i][j] = et - dist[i][j];
            }
        }
    }
    let dx = vec![0, 1, 2, 1, 0, -1, -2, -1];
    let dy = vec![2, 1, 0, -1, -2, -1, 0, 1];
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if dist[i][j] < MAX {
                for k in 0..8 {
                    let pt = (i as i32 + dx[k], j as i32 + dy[k]);
                    if valid(pt.0 as i32, pt.1 as i32) && edist[pt.0 as usize][pt.1 as usize] < MAX
                    {
                        if dist[i][j] + edist[pt.0 as usize][pt.1 as usize] + 2 <= et - 100 {
                            ans += 1;
                        }
                    }
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
