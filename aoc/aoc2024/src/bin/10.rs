use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 01:29:06 PM(13:29:06) IST(+05:30) 10-12-2024 Tue
*/

fn valid(x: i32, y: i32, h: i32, w: i32) -> bool {
    return x >= 0 && x < h && y >= 0 && y < w;
}

fn dfs(
    pt: (i32, i32),
    t: &Vec<Vec<i32>>,
    seen: &mut Vec<Vec<bool>>,
    h: i32,
    w: i32,
    paths: &mut i32,
) {
    seen[pt.0 as usize][pt.1 as usize] = true;
    if t[pt.0 as usize][pt.1 as usize] == 9 {
        *paths += 1;
        return;
    }
    let dx = vec![1, 0, 0, -1];
    let dy = vec![0, 1, -1, 0];
    for i in 0..4usize {
        let x = pt.0 + dx[i];
        let y = pt.1 + dy[i];
        if valid(x, y, h, w)
            && !seen[x as usize][y as usize]
            && t[x as usize][y as usize] == t[pt.0 as usize][pt.1 as usize] + 1
        {
            dfs((x, y), t, seen, h, w, paths);
        }
    }
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut t = Vec::<Vec<i32>>::new();
    while let Some(x) = sc.try_next() {
        // dbg!(x);
        let tt = x.chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
        t.push(tt);
    }
    let h = t.len();
    let w = t[0].len();
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if t[i][j] == 0 {
                let mut seen = vec![vec![false; w]; h];
                let mut paths = 0;
                dfs(
                    (i as i32, j as i32),
                    &t,
                    &mut seen,
                    h as i32,
                    w as i32,
                    &mut paths,
                );
                ans += paths;
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
