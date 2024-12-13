use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 11:32:28 PM(23:32:28) IST(+05:30) 13-12-2024 Fri
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut a = Vec::<(i128, i128)>::new();
    let mut b = Vec::<(i128, i128)>::new();
    let mut p = Vec::<(i128, i128)>::new();
    while let Some(s) = sc.try_next() {
        if s.len() > 0 {
            let s1 = s.split(':').collect::<Vec<&str>>()[1].trim();
            let mut s2 = s1.split(',').collect::<Vec<&str>>();
            assert!(
                s2.len() == 2,
                "More than 2 coordinates is not possible. Some parsing bug!"
            );
            for i in 0..s2.len() {
                s2[i] = s2[i].trim();
            }
            if s.starts_with("Button A") {
                a.push((s2[0][2..].parse().unwrap(), s2[1][2..].parse().unwrap()));
            } else if s.starts_with("Button B") {
                b.push((s2[0][2..].parse().unwrap(), s2[1][2..].parse().unwrap()));
            } else {
                p.push((s2[0][2..].parse().unwrap(), s2[1][2..].parse().unwrap()));
            }
        }
    }
    let mut ans = 0;
    for i in 0..a.len() {
        let ax = a[i].0;
        let ay = a[i].1;
        let bx = b[i].0;
        let by = b[i].1;
        let x = p[i].0 + 10_000_000_000_000;
        let y = p[i].1 + 10_000_000_000_000;
        let n_numr = bx * y - by * x;
        let n_denr = bx * ay - by * ax;
        let m_numr = ax * y - ay * x;
        let m_denr = ax * by - ay * bx;
        if m_numr % m_denr == 0 && n_numr % n_denr == 0 {
            ans += (n_numr / n_denr) * 3 + (m_numr / m_denr) * 1;
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
