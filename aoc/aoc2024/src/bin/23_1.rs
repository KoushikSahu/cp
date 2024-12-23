use std::collections::BTreeSet;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 06:43:15 PM(18:43:15) IST(+05:30) 23-12-2024 Mon
*/

fn recurse(
    node: i32,
    g: &Vec<Vec<i32>>,
    seen: BTreeSet<i32>,
    cliques: &mut BTreeSet<BTreeSet<i32>>,
) {
    if cliques.contains(&seen) {
        return;
    }
    cliques.insert(seen.clone());
    for &neighbour in g[node as usize].iter() {
        if seen.contains(&neighbour) {
            continue;
        }
        let mut inall = true;
        for s in seen.iter() {
            inall &= g[*s as usize].contains(&neighbour);
        }
        if !inall {
            continue;
        }
        let mut mod_seen = seen.clone();
        mod_seen.insert(neighbour);
        recurse(neighbour, g, mod_seen, cliques);
    }
}

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
        if !g[edge.0 as usize].contains(&edge.1) {
            g[edge.0 as usize].push(edge.1);
        }
        if !g[edge.1 as usize].contains(&edge.0) {
            g[edge.1 as usize].push(edge.0);
        }
    }
    let mut cliques = BTreeSet::<BTreeSet<i32>>::new();
    for v in 0..vertices.len() {
        let mut seen = BTreeSet::<i32>::new();
        seen.insert(v as i32);
        recurse(v as i32, &g, seen, &mut cliques);
    }
    let mx_clique = cliques.iter().max_by_key(|x| x.len()).unwrap();
    let mx_clique = mx_clique
        .iter()
        .map(|x| vertices[*x as usize].clone())
        .collect::<Vec<String>>();
    let ans = mx_clique.join(",");
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
    // let t: i32 = scan.next();
    let start_time = std::time::Instant::now();
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
    writeln!(out, "Elapsed time: {:?}", start_time.elapsed()).unwrap();
}
