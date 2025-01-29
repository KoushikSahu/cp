use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:50:21 PM(20:50:21) IST(+05:30) 29-01-2025 Wed
*/

struct SegmentTree {
    n: usize,
    t: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let t = vec![0; 2 * n];
        Self { n, t }
    }

    fn build(&mut self, a: &Vec<i32>) {
        for i in 0..self.n {
            self.t[self.n + i] = a[i];
        }
        for i in (0..self.n).rev() {
            self.t[i] = self.t[i << 1].max(self.t[i << 1 | 1]);
        }
    }

    fn query(&self, mut l: usize, mut r: usize) -> i32 {
        let mut res = 0;
        l += self.n;
        r += self.n;
        while l < r {
            if l & 1 != 0 {
                res = res.max(self.t[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                res = res.max(self.t[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let (n, p) = (sc.next::<i32>(), sc.next::<i32>());
    let a = (0..n).map(|_| sc.next::<i32>()).collect::<Vec<i32>>();
    let mut l = vec![-1; n as usize];
    let mut r = vec![n; n as usize];
    let mut x = -1;
    for i in 0..n {
        if a[i as usize] != 0 {
            l[i as usize] = x;
        } else {
            x = i;
        }
    }
    x = n;
    for i in (0..n).rev() {
        if a[i as usize] != 0 {
            r[i as usize] = x;
        } else {
            x = i;
        }
    }
    let mut st = SegmentTree::new(n as usize);
    st.build(&a);
    let mut ans = vec![0; n as usize];
    for i in 0..n as usize {
        if a[i] != 0 {
            let mut curr = i32::MAX;
            let lidx = l[i];
            if lidx != -1 {
                let tmp = st.query(lidx as usize, i + 1);
                curr = curr.min((tmp + p - 1) / p);
            }
            let ridx = r[i];
            if ridx != n {
                let tmp = st.query(i, ridx as usize + 1);
                curr = curr.min((tmp + p - 1) / p);
            }
            ans[i] = curr;
        }
    }
    writeln!(
        wr,
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();
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
