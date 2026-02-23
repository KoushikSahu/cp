use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:39:32 AM(08:39:32) IST(+05:30) 17-09-2025 Wed
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<i32>();
    let mut p = (0..n).map(|_| sc.next::<i32>()).collect::<Vec<i32>>();
    let mut seen = vec![false; n as usize];
    let (mut zero_cnt, mut zero_idx) = (0, 0);
    for i in 0..n {
        if p[i as usize] == 0 {
            zero_cnt += 1;
            zero_idx = i;
            continue;
        }
        seen[p[i as usize] as usize - 1] = true;
    }
    if zero_cnt == 1 {
        for i in 0..n {
            if seen[i as usize] == false {
                p[zero_idx as usize] = i + 1;
                break;
            }
        }
    }
    let (mut l, mut r) = (0, n-1);
    while l < n {
        if p[l as usize] == l + 1 {
            l += 1;
        } else {
            break;
        }
    }
    while r >= 0 {
        if p[r as usize] == r + 1 {
            r -= 1;
        } else {
            break;
        }
    }
    if l == n {
        writeln!(wr, "0").unwrap();
        return;
    }
    writeln!(wr, "{}", r - l + 1).unwrap();
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
