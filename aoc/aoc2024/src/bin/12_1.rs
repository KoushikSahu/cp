use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 01:21:34 PM(13:21:34) IST(+05:30) 12-12-2024 Thu
*/

fn valid(pt: (i32, i32), sz: (i32, i32)) -> bool {
    return pt.0 >= 0 && pt.0 < sz.0 && pt.1 >= 0 && pt.1 < sz.1;
}

fn dfs(
    pt: (usize, usize),
    f: &Vec<Vec<char>>,
    seen: &mut Vec<Vec<bool>>,
    p: &mut i32,
    a: &mut i32,
) {
    if seen[pt.0][pt.1] {
        return;
    }
    *a += 1;
    seen[pt.0][pt.1] = true;
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, 1, 0, -1];
    let h = f.len();
    let w = f[0].len();
    for i in 0..4 {
        let x = pt.0 as i32 + dx[i];
        let y = pt.1 as i32 + dy[i];
        if valid((x, y), (h as i32, w as i32)) && f[x as usize][y as usize] == f[pt.0][pt.1] {
            dfs((x as usize, y as usize), f, seen, p, a);
        }
    }
    for i in 0..4 {
        let i1 = i;
        let i2 = if i + 1 < 4 { i + 1 } else { 0 };
        let x1 = pt.0 as i32 + dx[i1];
        let y1 = pt.1 as i32 + dy[i1];
        let x2 = pt.0 as i32 + dx[i2];
        let y2 = pt.1 as i32 + dy[i2];
        let xd = pt.0 as i32 + dx[i1] + dx[i2];
        let yd = pt.1 as i32 + dy[i1] + dy[i2];
        let mut crnr = false;
        if (!valid((x1, y1), (h as i32, w as i32)) || f[x1 as usize][y1 as usize] != f[pt.0][pt.1])
            && (!valid((x2, y2), (h as i32, w as i32))
                || f[x2 as usize][y2 as usize] != f[pt.0][pt.1])
        {
            crnr = true;
        }
        if valid((x1, y1), (h as i32, w as i32))
            && valid((x2, y2), (h as i32, w as i32))
            && f[x1 as usize][y1 as usize] == f[pt.0][pt.1]
            && f[x2 as usize][y2 as usize] == f[pt.0][pt.1]
            && f[xd as usize][yd as usize] != f[pt.0][pt.1]
        {
            crnr = true;
        }
        if crnr {
            *p += 1;
        }
    }
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut f = Vec::<Vec<char>>::new();
    while let Some(s) = sc.try_next() {
        f.push(s.chars().collect());
    }
    let h = f.len();
    let w = f[0].len();
    let mut ans = 0;
    let mut seen = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if !seen[i][j] {
                let mut p = 0;
                let mut a = 0;
                dfs((i, j), &f, &mut seen, &mut p, &mut a);
                ans += p * a;
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
