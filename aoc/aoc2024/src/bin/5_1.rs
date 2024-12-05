use std::collections::{BTreeMap, VecDeque};
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::iter::zip;
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:43:58 PM(20:43:58) IST(+05:30) 05-12-2024 Thu
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut upds: Vec<Vec<i32>> = Vec::new();
    while let Some(s) = sc.try_next::<String>() {
        let rule: Vec<&str> = s.split('|').collect();
        if rule.len() == 2 {
            let a: i32 = rule[0].parse().unwrap();
            let b: i32 = rule[1].parse().unwrap();
            rules.push((a, b));
            continue;
        }
        let upd: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        upds.push(upd);
    }
    let nxm: i32 = 105;
    let mut ok: Vec<Vec<bool>> = vec![vec![true; nxm as usize]; nxm as usize];
    for rule in rules.iter() {
        ok[rule.1 as usize][rule.0 as usize] = false;
    }
    let mut ans: i32 = 0;
    for upd in upds.iter_mut() {
        let mut flipped = false;
        for i in 1..upd.len() {
            for j in 0..i {
                let is_ok = ok[upd[j] as usize][upd[i] as usize];
                if !is_ok {
                    flipped = true;
                    upd.swap(i, j);
                }
            }
        }
        if flipped {
            ans += upd[upd.len() / 2];
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
