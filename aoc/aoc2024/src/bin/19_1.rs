use std::collections::BTreeMap;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 01:05:11 PM(13:05:11) IST(+05:30) 19-12-2024 Thu
*/

#[derive(Debug)]
struct Possible {
    ok: bool,
    possibilities: i64,
}

fn possible(s: String, opts: &Vec<String>, dp: &mut BTreeMap<String, Possible>) -> (bool, i64) {
    if s.len() == 0 {
        return (true, 1);
    }
    if dp.contains_key(&s) {
        return (dp[&s].ok, dp[&s].possibilities);
    }
    let mut ok = false;
    let mut ps = 0i64;
    for opt in opts.iter() {
        if s.len() >= opt.len() {
            if &s[..opt.len()] == *opt {
                let res = possible(s[opt.len()..].to_owned(), opts, dp);
                if res.0 {
                    ok = true;
                    ps += res.1;
                }
            }
        }
    }
    dp.insert(
        s,
        Possible {
            ok,
            possibilities: ps,
        },
    );
    return (ok, ps);
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut opts = Vec::<String>::new();
    let mut patterns = Vec::<String>::new();
    while let Some(s) = sc.try_next() {
        let s1 = s.split(", ").collect::<Vec<&str>>();
        if s1.len() == 1 {
            if s1[0].len() > 0 {
                patterns.push(s1[0].to_owned());
            }
        } else {
            for ss in s1 {
                opts.push(ss.to_owned());
            }
        }
    }
    let mut ans = 0;
    let mut dp = BTreeMap::<String, Possible>::new();
    for p in patterns {
        let res = possible(p.clone(), &opts, &mut dp);
        if res.0 {
            ans += res.1;
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
