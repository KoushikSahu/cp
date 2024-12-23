use std::collections::BTreeSet;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 06:43:15 PM(18:43:15) IST(+05:30) 23-12-2024 Mon
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut edges = Vec::<(String, String)>::new();
    let mut vertices = BTreeSet::<String>::new();
    while let Some(s) = sc.try_next() {
        let s1 = s.split('-').collect::<Vec<&str>>();
        edges.push((s1[0].to_string(), s1[1].to_string()));
        vertices.insert(s1[0].to_string());
        vertices.insert(s1[1].to_string());
    }
    let vertices = vertices.into_iter().collect::<Vec<String>>();
    let edges = edges
        .into_iter()
        .map(|(a, b)| {
            (
                vertices.iter().position(|x| x == &a).unwrap() as i32,
                vertices.iter().position(|x| x == &b).unwrap() as i32,
            )
        })
        .collect::<Vec<(i32, i32)>>();
    let mut g = vec![Vec::<i32>::new(); vertices.len()];
    for edge in edges {
        g[edge.0 as usize].push(edge.1);
        g[edge.1 as usize].push(edge.0);
    }
    let mut ans = 0;
    for v1 in 0..vertices.len() - 2 {
        for v2 in v1 + 1..vertices.len() - 1 {
            for v3 in v2 + 1..vertices.len() {
                if g[v1].contains(&(v2 as i32))
                    && g[v2].contains(&(v3 as i32))
                    && g[v3].contains(&(v1 as i32))
                    && (vertices[v1].starts_with('t')
                        || vertices[v2].starts_with('t')
                        || vertices[v3].starts_with('t'))
                {
                    ans += 1;
                }
            }
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
