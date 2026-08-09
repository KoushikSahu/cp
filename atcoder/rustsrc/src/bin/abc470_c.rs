use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;
use std::collections::BTreeSet;

/*
    Author: Koushik Sahu
    Created: 03:38:57 PM(15:38:57) IST(+05:30) 09-08-2026 Sun
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let (_n, q) = (sc.next::<i32>(), sc.next::<i32>());
    let nxm = 500_005;
    let mut a = vec![0; nxm];
    let mut ans = 0;
    let mut idxs = BTreeSet::<i32>::new();
    for _ in 0..q {
        let t = sc.next::<i32>();
        match t {
            1 => {
                let val = sc.next::<i32>();
                if a[val as usize] == 0 {
                    idxs.insert(val);
                }
                ans ^= a[val as usize];
                ans ^= a[val as usize] + 1;
                a[val as usize] += 1;
                writeln!(wr, "{}", ans).unwrap();
            },
            2 => {
                ans = 0;
                let mut tmp = BTreeSet::<i32>::new();
                for &i in idxs.iter() {
                    a[i as usize] -= 1;
                    ans ^= a[i as usize];
                    if a[i as usize] >= 1 {
                        tmp.insert(i as i32);
                    }
                }
                idxs = tmp;
                writeln!(wr, "{}", ans).unwrap();
            },
            _ => ()
        }
    }
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
