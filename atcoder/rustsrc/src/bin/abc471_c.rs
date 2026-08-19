use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 05:54:24 PM(17:54:24) IST(+05:30) 15-08-2026 Sat
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<usize>();
    let mut a = (0..n).map(|_| sc.next::<i64>()).collect::<Vec<_>>();
    a.push(0);
    a.sort();
    let mut curr_idx: i32 = 0;
    for i in 0..a.len() {
        if a[i] == 0 {
            curr_idx = i as i32;
        }
    }
    let (mut l, mut r) = (curr_idx - 1, curr_idx + 1);
    let mut ans = 0i64;
    for _ in 0..n {
        if l < 0 {
            ans += (a[curr_idx as usize] - a[r as usize]).abs();
            curr_idx = r;
            r += 1;
        } else if r > a.len() as i32 - 1 {
            ans += (a[curr_idx as usize] - a[l as usize]).abs();
            curr_idx = l;
            l -= 1;
        } else {
            let ld = (a[curr_idx as usize] - a[l as usize]).abs();
            let rd = (a[curr_idx as usize] - a[r as usize]).abs();
            if ld <= rd {
                ans += ld;
                curr_idx = l;
                l -= 1;
            } else {
                ans += rd;
                curr_idx = r;
                r += 1;
            }
        }
    }
    writeln!(wr, "{}", ans).ok();
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
