use std::collections::LinkedList;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 01:26:36 PM(13:26:36) IST(+05:30) 09-12-2024 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let s = sc
        .next::<String>()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let mut fs = Vec::<(u32, u32)>::new(); // contains id and size
    let mut sp = Vec::<(u32, Vec<u32>, Vec<u32>)>::new(); // contains size and list of files
    for i in 0..s.len() {
        if i % 2 == 0 {
            fs.push((i as u32 / 2, s[i]));
        } else {
            sp.push((s[i], Vec::<u32>::new(), Vec::<u32>::new()));
        }
    }
    for i in (0..fs.len()).rev() {
        for j in 0..sp.len() {
            if j < i && fs[i].1 <= sp[j].0 {
                sp[j].1.push(fs[i].0);
                sp[j].2.push(fs[i].1);
                sp[j].0 -= fs[i].1;
                sp[i - 1].0 += fs[i].1;
                fs[i].1 = 0;
                break;
            }
        }
    }
    // dbg!(fs, sp);
    let mut i = 0;
    let mut j = 0;
    let mut efs = Vec::<i32>::new();
    loop {
        if i < fs.len() {
            for _ in 0..fs[i].1 {
                efs.push(fs[i].0 as i32);
            }
        }
        // dbg!(efs);
        if j < sp.len() {
            for k in 0..sp[j].1.len() {
                for _ in 0..sp[j].2[k] {
                    efs.push(sp[j].1[k] as i32);
                }
            }
            for _ in 0..sp[j].0 {
                efs.push(-1);
            }
        } else {
            break;
        }
        // dbg!(efs);
        i += 1;
        j += 1;
    }
    let mut ans = 0i64;
    for i in 0..efs.len() {
        if efs[i] != -1 {
            ans += efs[i] as i64 * i as i64;
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
            return unsafe { Some(str::from_utf8_unchecked(&self.buf_str).to_owned()) };
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
