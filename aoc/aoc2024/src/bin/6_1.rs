use std::collections::HashSet;
use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;
use std::time::{Duration, Instant};

/*
    Author: Koushik Sahu
    Created: 06:46:56 PM(18:46:56) IST(+05:30) 06-12-2024 Fri
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut field: Vec<Vec<char>> = Vec::new();
    while let Some(s) = sc.try_next::<String>() {
        field.push(s.chars().collect());
    }
    let mut g: (Option<i32>, Option<i32>) = (None, None);
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == '>' || field[i][j] == '^' || field[i][j] == '<' || field[i][j] == 'v'
            {
                g = (Some(i as i32), Some(j as i32));
            }
        }
    }
    // dbg!(g);
    let get_direction = |c: char| -> (i32, i32) {
        match c {
            '>' => (0, 1),
            '<' => (0, -1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => (0, 0),
        }
    };
    let rotate_clockwise = |c: char| -> char {
        match c {
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            '^' => '>',
            _ => c,
        }
    };
    let h = field.len() as i32;
    let w = field[0].len() as i32;
    let valid = |i: i32, j: i32| -> bool { i >= 0 && i < h && j >= 0 && j < w };
    // dbg!(h, w);
    let field_clone = field.clone();
    let g_clone = g.clone();
    let mut ans: i32 = 0;
    for i in 0..h as usize {
        for j in 0..w as usize {
            // dbg!(i, j);
            if field[i][j] == '.' {
                field[i][j] = '#';
                let mut seen = HashSet::<(i32, i32, char)>::new();
                loop {
                    let mut dc = field[g.0.unwrap() as usize][g.1.unwrap() as usize];
                    let d = get_direction(dc);
                    let mut end = false;
                    loop {
                        let x = g.0.unwrap();
                        let y = g.1.unwrap();
                        let xx = x + d.0;
                        let yy = y + d.1;
                        // dbg!(x, y, xx, yy);
                        if valid(xx, yy) {
                            if field[xx as usize][yy as usize] != '#' {
                                g.0 = Some(xx);
                                g.1 = Some(yy);
                            } else {
                                break;
                            }
                        } else {
                            end = true;
                            break;
                        }
                    }

                    if seen.contains(&(
                        g.0.unwrap(),
                        g.1.unwrap(),
                        field[g.0.unwrap() as usize][g.1.unwrap() as usize],
                    )) {
                        // dbg!(
                        //     i,
                        //     j,
                        //     g.0.unwrap(),
                        //     g.1.unwrap(),
                        //     field[g.0.unwrap() as usize][g.1.unwrap() as usize]
                        // );
                        ans += 1;
                        break;
                    } else {
                        seen.insert((
                            g.0.unwrap(),
                            g.1.unwrap(),
                            field[g.0.unwrap() as usize][g.1.unwrap() as usize],
                        ));
                    }

                    if end {
                        break;
                    }

                    dc = rotate_clockwise(dc);
                    field[g.0.unwrap() as usize][g.1.unwrap() as usize] = dc;
                }
                field = field_clone.clone();
                g = g_clone;
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
    fn try_next<T: str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok().expect("Failed parse"));
            }
            self.buf_str.clear();
            let bytes_read = self
                .reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            if bytes_read == 0 {
                return None; // End of file reached
            }
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
    //let t: i32 = scan.next();
    for _ in 0..t {
        solve(&mut scan, &mut out);
    }
}
