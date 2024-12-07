use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;
use std::time::Instant;

/*
    Author: Koushik Sahu
    Created: 12:40:22 PM(12:40:22) IST(+05:30) 07-12-2024 Sat
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut ans = 0i64;
    while let Some(s) = sc.try_next() {
        let colon_split = s.split(":").collect::<Vec<&str>>();
        let res = colon_split[0]
            .parse::<i64>()
            .expect("Failed to parse the expected result.");
        // dbg!(colon_split[1]);
        let nums = colon_split[1]
            .trim()
            .split(' ')
            .map(|x| {
                // dbg!(x);
                x.parse::<i64>()
                    .expect("Failed to parse candidate numbers.")
            })
            .collect::<Vec<i64>>();
        // dbg!(res, nums);
        let n = nums.len() as u32;
        let mut valid = false;
        for i in 0..(3i64.pow(n - 1)) {
            let mut tmp = i;
            let mut curr = nums[0];
            for j in 0..n - 1 {
                if tmp % 3 == 0 {
                    curr += nums[j as usize + 1];
                } else if tmp % 3 == 1 {
                    curr *= nums[j as usize + 1];
                } else {
                    let mut nxt_tmp = nums[j as usize + 1];
                    while nxt_tmp > 0 {
                        curr *= 10;
                        nxt_tmp /= 10;
                    }
                    curr += nums[j as usize + 1];
                }
                tmp /= 3;
            }
            if curr == res {
                valid = true;
                break;
            }
        }
        if valid {
            ans += res;
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
    //let t: i32 = scan.next(); jj
    let start = Instant::now();
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
    writeln!(out, "Execution time: {:?}", start.elapsed()).unwrap();
}
