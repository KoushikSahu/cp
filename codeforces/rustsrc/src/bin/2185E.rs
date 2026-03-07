use std::collections::BTreeMap;
use std::io::{stdin, stdout, BufWriter, prelude::BufRead, prelude::Write};
use std::str;

/*
    Author: Koushik Sahu
    Created: 02:51:40 AM(02:51:40) IST(+05:30) 01-03-2026 Sun
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let (n, m, k) = (sc.next::<i32>(), sc.next::<i32>(), sc.next::<i32>());
    let a = (0..n).map(|_| sc.next::<i32>()).collect::<Vec<i32>>();
    let mut b = (0..m).map(|_| sc.next::<i32>()).collect::<Vec<i32>>();
    b.sort();
    let k = sc.next::<String>().chars().collect::<Vec<char>>();
    let mut mp = BTreeMap::<i32, Vec<i32>>::new();
    for i in 0..n {
        let (mut low, mut high) = (0, m);
        while low < high {
            let mid = low + (high - low) / 2;
            if b[mid as usize] <= a[i as usize] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        if low < m {
            let dr = b[low as usize] - a[i as usize];
            mp.entry(dr).and_modify(|x| x.push(i)).or_insert(vec![i]);
        }
        if low - 1 >= 0 {
            let dl = a[i as usize] - b[low as usize - 1];
            mp.entry(-dl).and_modify(|x| x.push(i)).or_insert(vec![i]);
        }
    }
    let mut dead = vec![false; n as usize];
    let mut dc = 0;
    let mut ans = n;
    for &i in k.iter() {
        match i {
            'L' => dc -= 1,
            'R' => dc += 1,
            _ => ()
        }
        if mp.contains_key(&dc) {
            for &j in mp[&dc].iter() {
                if !dead[j as usize] {
                    dead[j as usize] = true;
                    ans -= 1;
                }
            }
            if let Some(v) = mp.get_mut(&dc) {
                v.clear();
            }
        }
        write!(wr, "{} ", ans).unwrap();
    }
    writeln!(wr).unwrap();
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
