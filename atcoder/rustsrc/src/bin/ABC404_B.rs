use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 07:30:11 PM(19:30:11) IST(+05:30) 04-05-2025 Sun
*/

fn edit_distance(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> i32 {
    let n = s.len();
    let mut ed = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[i][j] {
                ed += 1;
            }
        }
    }
    ed
}

fn rotate(s: &mut Vec<Vec<char>>) {
    let n = s.len();
    for i in 0..n {
        for j in 0..n {
            if i < j {
                (s[i][j], s[j][i]) = (s[j][i], s[i][j]);
            }
        }
    }
    for i in 0..n {
        let (mut l, mut h) = (0, n - 1);
        while l < h {
            (s[i][l], s[i][h]) = (s[i][h], s[i][l]);
            l += 1;
            h -= 1;
        }
    }
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<usize>();
    let mut s: Vec<Vec<char>> = (0..n)
        .map(|_| sc.next::<String>().chars().collect())
        .collect();
    let mut t: Vec<Vec<char>> = (0..n)
        .map(|_| sc.next::<String>().chars().collect())
        .collect();
    let mut ans = i32::MAX;
    for i in 0..=4 {
        // writeln!(wr).unwrap();
        // for i in 0..n {
        //     for j in 0..n {
        //         write!(wr, "{}", s[i][j]).unwrap();
        //     }
        //     writeln!(wr).unwrap();
        // }
        ans = ans.min(i + edit_distance(&s, &t));
        rotate(&mut s);
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
