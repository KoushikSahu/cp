use std::collections::{BTreeMap, BinaryHeap};
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 12:23:52 PM(12:23:52) IST(+05:30) 16-12-2024 Mon
*/

fn opposite(dd: (i32, i32), d: (i32, i32)) -> bool {
    dd.0 + d.0 == 0 && dd.1 + d.1 == 0
}

fn valid(pn: (i32, i32), h: i32, w: i32) -> bool {
    pn.0 >= 0 && pn.0 < h && pn.1 >= 0 && pn.1 < w
}

fn right_angle(dd: (i32, i32), d: (i32, i32)) -> bool {
    dd.0 * d.0 + dd.1 * d.1 == 0
}

fn recurse(
    prev: &Vec<Vec<Vec<Vec<Vec<((i32, i32), (i32, i32))>>>>>,
    sp: &mut Vec<Vec<bool>>,
    pt: (i32, i32),
    d: (i32, i32),
) {
    sp[pt.0 as usize][pt.1 as usize] = true;
    if prev[pt.0 as usize][pt.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize].is_empty() {
        return;
    }
    for &p in &prev[pt.0 as usize][pt.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize] {
        recurse(prev, sp, (p.0 .0, p.0 .1), (p.1 .0, p.1 .1));
    }
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut mz = Vec::<Vec<char>>::new();
    while let Some(s) = sc.try_next() {
        mz.push(s.chars().collect());
    }
    let mut s = (-1, -1);
    let mut e = (-1, -1);
    let h = mz.len();
    let w = mz[0].len();
    for i in 0..h {
        for j in 0..w {
            if mz[i][j] == 'S' {
                s = (i as i32, j as i32);
            } else if mz[i][j] == 'E' {
                e = (i as i32, j as i32);
            }
        }
    }
    let dx = vec![1, -1, 0, 0];
    let dy = vec![0, 0, -1, 1];
    let mut bh = BinaryHeap::<(i64, (i32, i32), (i32, i32))>::new();
    bh.push((0, s, (0, 1)));
    let mut dist = vec![vec![vec![vec![i64::MAX; 3]; 3]; w]; h];
    let mut prev = vec![vec![vec![vec![vec![]; 3]; 3]; w]; h];
    dist[s.0 as usize][s.1 as usize][0 + 1][1 + 1] = 0;
    while !bh.is_empty() {
        let n = bh.pop().unwrap();
        for i in 0..4 {
            let d = (dx[i], dy[i]);
            if opposite(d, n.2) {
                continue;
            }
            let pn = (n.1 .0 + d.0, n.1 .1 + d.1);
            if valid(pn, h as i32, w as i32) && mz[pn.0 as usize][pn.1 as usize] != '#' {
                let curr_dist = n.0 + 1 + if right_angle(d, n.2) { 1000 } else { 0 };
                if dist[pn.0 as usize][pn.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize]
                    > curr_dist
                {
                    dist[pn.0 as usize][pn.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize] =
                        curr_dist;
                    prev[pn.0 as usize][pn.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize] =
                        vec![(n.1, n.2)];
                    bh.push((curr_dist, pn, d));
                } else if dist[pn.0 as usize][pn.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize]
                    == curr_dist
                {
                    prev[pn.0 as usize][pn.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize]
                        .push((n.1, n.2));
                }
            }
        }
    }
    let mut sp = vec![vec![false; w]; h];
    let mut lowest_direction = Vec::<(i32, i32)>::new();
    let mut mn = i64::MAX;
    for i in 0..4 {
        let d = (dx[i], dy[i]);
        if dist[e.0 as usize][e.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize] < mn {
            mn = dist[e.0 as usize][e.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize];
            lowest_direction = vec![d];
        } else if dist[e.0 as usize][e.1 as usize][(d.0 + 1) as usize][(d.1 + 1) as usize] == mn {
            lowest_direction.push(d);
        }
    }
    for d in lowest_direction.iter() {
        recurse(&prev, &mut sp, e, *d);
    }
    let mut ans = 0;
    for i in 0..sp.len() {
        for j in 0..sp[0].len() {
            write!(wr, "{}", if sp[i][j] { 'O' } else { mz[i][j] }).unwrap();
            if sp[i][j] {
                ans += 1;
            }
        }
        writeln!(wr).unwrap();
    }
    writeln!(wr).unwrap();
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
