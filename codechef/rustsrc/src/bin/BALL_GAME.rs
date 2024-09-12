use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:46:58 PM(20:46:58) IST(+05:30) 11-09-2024 Wed
*/

#[derive(Debug)]
struct ball {
    position: i32,
    speed: i32,
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n: i32 = sc.next();
    let a: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    let b: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    let mut balls: Vec<ball> = Vec::new();
    for i in 0..n as usize {
        balls.push(ball{position: a[i], speed: b[i]});
    }
    balls.sort_by(|a, b| {
        (&a.position).cmp(&b.position)
    });
    let mut ans: i32 = 1;
    let mut bl = &balls[n as usize-1];
    for i in (0..n as usize - 1).rev() {
        if i as i32 == n-1 {
            continue;
        }
        let lhs: i64 = balls[i].position as i64 * (bl.speed as i64 - balls[i].speed as i64);
        let rhs: i64 = balls[i].speed as i64 * (bl.position as i64 - balls[i].position as i64);
        if lhs <= rhs {
            ans += 1;
            bl = &balls[i];
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
