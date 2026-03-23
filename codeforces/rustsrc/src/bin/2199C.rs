use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 11:21:05 PM(23:21:05) IST(+05:30) 23-03-2026 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let k = sc.next::<i32>();
    let nxm = 3 * k as usize;
    let mut a = vec![vec!['.'; nxm]; 2];
    let mut b = vec![vec!['.'; nxm]; 2];
    for i in 0..nxm {
        if (((i as i32 - 1) % 3) + 3) % 3 == 0 {
            a[0][i] = '*';
        } else if (i % 3 == 0) {
            b[0][i] = '*';
        }
    }
    let mut aa = vec![0; nxm];
    let mut bb = vec![0; nxm];
    for i in 0..nxm {
        if a[0][i] == '*' {
            if i > 0 {
                aa[i] = aa[i - 1] + 3;
            } else {
                aa[i] = 1;
            }
        } else {
            if i > 0 {
                if a[0][i - 1] == '*' {
                    aa[i] = aa[i - 1] + 2;
                } else {
                    aa[i] = aa[i - 1];
                }
            } else {
                aa[i] = 0;
            }
        }
        if b[0][i] == '*' {
            if i > 0 {
                bb[i] = bb[i - 1] + 3;
            } else {
                bb[i] = 1;
            }
        } else {
            if i > 0 {
                if b[0][i - 1] == '*' {
                    bb[i] = bb[i - 1] + 2;
                } else {
                    bb[i] = bb[i - 1];
                }
            } else {
                bb[i] = 0;
            }
        }
    }
    let (mut low, mut high) = (0, nxm);
    while (low < high) {
        let mid = low + (high - low) / 2;
        if (aa[mid] < k) {
            low = mid + 1;
        } else { 
            high = mid;
        }
    }
    if low < nxm && aa[low] == k {
        writeln!(wr, "YES").unwrap();
        writeln!(wr, "{}", low + 1).unwrap();
        writeln!(wr, "{}", a[0][0..low + 1].iter().collect::<String>()).unwrap();
        writeln!(wr, "{}", a[1][0..low + 1].iter().collect::<String>()).unwrap();
        return;
    }
    low = 0;
    high = nxm;
    while (low < high) {
        let mid = low + (high - low) / 2;
        if (bb[mid] < k) {
            low = mid + 1;
        } else { 
            high = mid;
        }
    }
    if low < nxm && bb[low] == k {
        writeln!(wr, "YES").unwrap();
        writeln!(wr, "{}", low + 1).unwrap();
        writeln!(wr, "{}", b[0][0..low + 1].iter().collect::<String>()).unwrap();
        writeln!(wr, "{}", b[1][0..low + 1].iter().collect::<String>()).unwrap();
        return;
    }
    writeln!(wr, "NO").unwrap();
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
