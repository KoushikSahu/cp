use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:13:35 PM(20:13:35) IST(+05:30) 29-01-2025 Wed
*/

fn get_rest(a: char, b: char) -> char {
    if a == 'R' && b == 'G' || a == 'G' && b == 'R' {
        return 'B';
    }
    if a == 'R' && b == 'B' || a == 'B' && b == 'R' {
        return 'G';
    }
    return 'R';
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n = sc.next::<i32>();
    let s = sc.next::<String>().chars().collect::<Vec<char>>();
    let mut ps = vec![
        vec!['R', 'G', 'B'],
        vec!['R', 'B', 'G'],
        vec!['G', 'R', 'B'],
        vec!['G', 'B', 'R'],
        vec!['B', 'R', 'G'],
        vec!['B', 'G', 'R'],
    ];
    for i in 0..6 {
        while ps[i].len() < n as usize {
            let l = ps[i][ps[i].len() - 1];
            let sl = ps[i][ps[i].len() - 2];
            let r = get_rest(l, sl);
            ps[i].push(r);
        }
    }
    let mut min_dist = i32::MAX;
    for i in 0..6 {
        let mut dist = 0;
        for j in 0..n as usize {
            if ps[i][j] != s[j] {
                dist += 1;
            }
        }
        min_dist = min_dist.min(dist);
    }
    writeln!(wr, "{}", min_dist).unwrap();
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
