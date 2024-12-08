use std::collections::{BTreeMap, BTreeSet};
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 12:44:28 PM(12:44:28) IST(+05:30) 08-12-2024 Sun
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut s = Vec::<Vec<char>>::new();
    while let Some(ss) = sc.try_next() {
        let sss = ss.trim();
        s.push(sss.chars().collect());
    }
    let mut mp = BTreeMap::<char, Vec<(i32, i32)>>::new();
    let h = s.len();
    let w = s[0].len();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '.' {
                mp.entry(s[i][j])
                    .and_modify(|x| x.push((i as i32, j as i32)))
                    .or_insert(vec![(i as i32, j as i32)]);
            }
        }
    }
    let mut an = BTreeSet::<(i32, i32)>::new();
    let dist = |x: (i32, i32), y: (i32, i32)| -> i32 { (x.0 - y.0).abs() + (x.1 - y.1).abs() };
    let inline = |x: (i32, i32), y: (i32, i32), z: (i32, i32)| -> bool {
        (y.1 - x.1) * (z.0 - x.0) == (z.1 - x.1) * (y.0 - x.0)
    };
    for a in mp.keys() {
        let pts = mp.get(a).unwrap();
        if pts.len() > 1 {
            for a1 in 0..pts.len() - 1 {
                for a2 in a1 + 1..pts.len() {
                    let pt1 = pts[a1];
                    let pt2 = pts[a2];
                    for i in 0..h {
                        for j in 0..w {
                            if inline(pt1, pt2, (i as i32, j as i32)) {
                                an.insert((i as i32, j as i32));
                            }
                        }
                    }
                }
            }
        }
    }
    writeln!(wr, "{}", an.len()).unwrap();
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
            return unsafe { Some(str::from_utf8_unchecked(&self.buf_str).to_owned()) };
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
