use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 11:38:02 AM(11:38:02) IST(+05:30) 15-12-2024 Sun
*/

fn valid(p: (i32, i32), h: usize, w: usize) -> bool {
    return p.0 >= 0 && p.0 < h as i32 && p.1 >= 0 && p.1 < w as i32;
}

fn simulate(psn: &mut Vec<Vec<char>>, p: (i32, i32), dn: (i32, i32), c: char) -> bool {
    let pn = (p.0 + dn.0, p.1 + dn.1);
    let h = psn.len();
    let w = psn[0].len();
    if !valid(pn, h, w) {
        return false;
    }
    let mut pos = true;
    match psn[pn.0 as usize][pn.1 as usize] {
        '#' => pos = false,
        '[' => {
            if dn == (0, 1) {
                pos &= simulate(
                    psn,
                    (pn.0, pn.1 + 1),
                    dn,
                    psn[pn.0 as usize][pn.1 as usize + 1],
                ) && simulate(psn, pn, dn, psn[pn.0 as usize][pn.1 as usize]);
            } else {
                pos &= simulate(psn, pn, dn, psn[pn.0 as usize][pn.1 as usize])
                    && simulate(
                        psn,
                        (pn.0, pn.1 + 1),
                        dn,
                        psn[pn.0 as usize][pn.1 as usize + 1],
                    );
            }
        }
        ']' => {
            if dn == (0, -1) {
                pos &= simulate(
                    psn,
                    (pn.0, pn.1 - 1),
                    dn,
                    psn[pn.0 as usize][pn.1 as usize - 1],
                ) && simulate(psn, pn, dn, psn[pn.0 as usize][pn.1 as usize]);
            } else {
                pos &= simulate(psn, pn, dn, psn[pn.0 as usize][pn.1 as usize])
                    && simulate(
                        psn,
                        (pn.0, pn.1 - 1),
                        dn,
                        psn[pn.0 as usize][pn.1 as usize - 1],
                    );
            }
        }
        _ => (),
    }
    if pos {
        psn[pn.0 as usize][pn.1 as usize] = c;
        psn[p.0 as usize][p.1 as usize] = '.';
    }
    return pos;
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut psn = Vec::<Vec<char>>::new();
    let mut mv = String::new();
    while let Some(s) = sc.try_next() {
        if s.len() > 0 {
            let mut curr = Vec::<char>::new();
            for c in s.chars() {
                match c {
                    '#' => curr.extend(vec!['#', '#']),
                    'O' => curr.extend(vec!['[', ']']),
                    '.' => curr.extend(vec!['.', '.']),
                    '@' => curr.extend(vec!['@', '.']),
                    _ => (),
                }
            }
            psn.push(curr);
        } else {
            break;
        }
    }
    while let Some(s) = sc.try_next() {
        mv.extend(s.chars());
    }
    let h = psn.len();
    let w = psn[0].len();
    let mut rp = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if psn[i][j] == '@' {
                rp = (i as i32, j as i32);
            }
        }
    }
    for m in mv.chars() {
        let mut dn = (0, 0);
        match m {
            '<' => dn = (0, -1),
            '>' => dn = (0, 1),
            'v' => dn = (1, 0),
            '^' => dn = (-1, 0),
            _ => (),
        }
        let mut psn_cpy = psn.clone();
        let pos = simulate(&mut psn_cpy, rp, dn, psn[rp.0 as usize][rp.1 as usize]);
        if pos {
            psn = psn_cpy;
            rp = (rp.0 + dn.0, rp.1 + dn.1);
        }
    }
    for i in 0..psn.len() {
        for j in 0..psn[0].len() {
            write!(wr, "{}", psn[i][j]).unwrap();
        }
        writeln!(wr).unwrap();
    }
    writeln!(wr).unwrap();
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if psn[i][j] == '[' {
                ans += 100 * i + j;
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
