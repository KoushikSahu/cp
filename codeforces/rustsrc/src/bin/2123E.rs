use std::cmp::max;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:01:26 AM(08:01:26) IST(+05:30) 07-07-2025 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<i32>();
    let a = (0..n).map(|_| sc.next::<i32>()).collect::<Vec<i32>>();
    dbg!(a);
    let mut frq = vec![0i32; n as usize + 1];
    for i in a.iter() {
        frq[*i as usize] += 1;
    }
    let mut mex = n + 1;
    for i in 0..=n as usize {
        if frq[i] == 0 {
            mex = i as i32;
            break;
        }
    }
    if mex == 0 {
        for _ in 0..=n {
            write!(wr, "1 ").unwrap();
        }
        writeln!(wr).unwrap();
        return;
    }
    let mut excs = vec![0; mex as usize + 1];
    for i in 0..=mex as usize {
        if i <= n as usize {
            excs[i] = max(frq[i] - 1, 0);
        }
        if i > 0 {
            excs[i] += excs[i - 1];
        }
    }
    let mut mn = vec![0; mex as usize + 1];
    for i in (0..=mex).rev() {
        if i <= n {
            mn[i as usize] = frq[i as usize];
        }
    }
    for i in (0..n as usize).rev() {
        frq[i] += frq[i + 1];
    }
    let mut mx = vec![0; mex as usize + 1];
    for i in 0..=mex as usize {
        mx[i] = (if i > 0 { excs[i - 1] } else { 0 }) + frq[i];
    }
    let mut ans = vec![0; n as usize + 1];
    for i in 0..mex as usize + 1 {
        ans[mn[i] as usize] += 1;
        if mx[i] < n {
            ans[mx[i] as usize + 1] -= 1;
        }
    }
    write!(wr, "1 ").unwrap();
    for i in 1..=n as usize {
        ans[i] += ans[i - 1];
        write!(wr, "{} ", ans[i]).unwrap();
    }
    writeln!(wr).unwrap();
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
    let t: i32 = scan.next();
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
}
