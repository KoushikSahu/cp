use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:28:30 PM(20:28:30) IST(+05:30) 12-08-2026 Wed
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<i32>();
    let s = sc.next::<String>().chars().collect::<Vec<_>>();
    let mut curr = 0;
    let mut frqs = Vec::<i32>::new();
    if s[0] == '1' {
        frqs.push(curr)
    }
    for i in 0..n as usize {
        if i > 0 && s[i] != s[i - 1] {
            frqs.push(curr);
            curr = 1;
            continue;
        }
        curr += 1;
    }
    frqs.push(curr);
    if frqs.len() <= 2 || (s[0] == '1' && frqs.len() <= 3) {
        writeln!(wr, "{}", s.iter().collect::<String>()).ok();
        return;
    }
    let (mut mxzero, mut mnone) = (0, n);
    for i in (0..frqs.len()).step_by(2) {
        if i == frqs.len() - 1 {
            continue;
        }
        if frqs[i] > mxzero {
            mxzero = frqs[i];
            mnone = frqs[i + 1];
        } else if frqs[i] == mxzero {
            mnone = mnone.min(frqs[i + 1]);
        }
    }
    for _ in 0..mxzero {
        write!(wr, "0").ok();
    }
    for _ in 0..mnone {
        write!(wr, "1").ok();
    }
    writeln!(wr).ok();
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
