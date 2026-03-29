use std::collections::{BTreeMap, BTreeSet};
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 12:16:50 PM(12:16:50) IST(+05:30) 29-03-2026 Sun
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut n = sc.next::<i32>();
    let mut x = vec![Vec::<i32>::new(); 10];
    x[0].push(0);
    let mut mp = BTreeMap::<i32, Vec<i32>>::new();
    let numdigs = |mut x: i32| -> i32 {
        let mut d = 0;
        while x > 0 {
            d += 1;
            x /= 10;
        }
        d
    };
    for i in 0..31 {
        let pw = 2i32.pow(i);
        let d = numdigs(pw);
        mp.entry(d)
            .and_modify(|x| x.push(pw))
            .or_insert_with(|| vec![pw]);
    }
    let mut a = BTreeSet::<i32>::new();
    for i in 1..=9 {
        for j in 0..i {
            for k in 0..x[j].len() {
                let kk = x[j][k];
                let d = numdigs(kk);
                if let Some(pws) = mp.get(&(i as i32 - d)) {
                    for &pw in pws.iter() {
                        x[i].push(kk * 10i32.pow(i as u32 - d as u32) + pw);
                        a.insert(kk * 10i32.pow(i as u32 - d as u32) + pw);
                    }
                }
            }
        }
    }
    let mut ans = -1;
    for i in a.iter() {
        if n > 0 {
            ans = *i;
            n -= 1;
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
