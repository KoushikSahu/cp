use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 11:47:11 PM(23:47:11) IST(+05:30) 07-07-2024 Sun
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let n: i32 = sc.next();
    let s: Vec<char> = sc.next::<String>().chars().collect();
    if n < 3 {
        let mut ans = String::from("");
        ans.push(s[0]);
        ans.push(s[1]);
        write!(wr, "{}\n", ans.parse::<i32>().unwrap()).unwrap();
    } else if n == 3 {
        let first: i32 = s[..2].iter().collect::<String>().parse::<i32>().unwrap();
        let second: i32 = s[1..3].iter().collect::<String>().parse::<i32>().unwrap();
        let pos = vec![
            first + (s[2] as i32 - '0' as i32),
            first * (s[2] as i32 - '0' as i32),
            second + (s[0] as i32 - '0' as i32),
            second * (s[0] as i32 - '0' as i32),
        ];
        let ans = pos.iter().min().unwrap();
        write!(wr, "{}\n", ans).unwrap();
    } else {
        if s.contains(&'0') {
            write!(wr, "0\n").unwrap();
            return;
        }
        let mut ans = i32::MAX;
        for i in 0..n - 1 {
            let mut nums = Vec::<i32>::new();
            let mut curr: i32 = 0;
            for j in 0..n {
                if j == i {
                    nums.push(
                        s[i as usize..i as usize + 2]
                            .iter()
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap(),
                    );
                } else if j == i + 1 {
                    continue;
                } else {
                    nums.push(s[j as usize] as i32 - '0' as i32);
                }
            }
            for &num in nums.iter() {
                if num != 1 {
                    curr += num;
                }
            }
            dbg!(nums, curr);
            ans = ans.min(curr);
        }
        write!(wr, "{}\n", ans).unwrap();
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
