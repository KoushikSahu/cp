use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 11:38:02 AM(11:38:02) IST(+05:30) 15-12-2024 Sun
*/

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut psn = Vec::<Vec<char>>::new();
    let mut mv = String::new();
    while let Some(s) = sc.try_next() {
        if s.len() > 0 {
            psn.push(s.chars().collect());
        } else {
            break;
        }
    }
    while let Some(s) = sc.try_next() {
        mv.extend(s.chars());
    }
    let mut rp = (0, 0);
    for i in 0..psn.len() {
        for j in 0..psn[0].len() {
            if psn[i][j] == '@' {
                rp = (i as i32, j as i32);
            }
        }
    }
    let h = psn.len();
    let w = psn[0].len();
    let valid = |x, y| x >= 0 && x < h as i32 && y >= 0 && y < w as i32;
    for m in mv.chars() {
        let mut dn = (0, 0);
        match m {
            '<' => dn = (0, -1),
            '>' => dn = (0, 1),
            'v' => dn = (1, 0),
            '^' => dn = (-1, 0),
            _ => (),
        }
        let mut frst_bx: Option<(i32, i32)> = None;
        let mut nxt_fr: Option<(i32, i32)> = None;
        let mut nxt = (rp.0 + dn.0, rp.1 + dn.1);
        while valid(nxt.0, nxt.1) {
            match psn[nxt.0 as usize][nxt.1 as usize] {
                '.' => {
                    nxt_fr = Some((nxt.0, nxt.1));
                    break;
                }
                'O' => {
                    if frst_bx.is_none() {
                        frst_bx = Some((nxt.0, nxt.1));
                    }
                }
                '#' => {
                    break;
                }
                _ => (),
            }
            nxt = (nxt.0 + dn.0, nxt.1 + dn.1);
        }
        if let Some(fr) = nxt_fr {
            psn[rp.0 as usize][rp.1 as usize] = '.';
            if let Some(bx) = frst_bx {
                psn[bx.0 as usize][bx.1 as usize] = '@';
                psn[fr.0 as usize][fr.1 as usize] = 'O';
                rp = (bx.0, bx.1);
            } else {
                psn[fr.0 as usize][fr.1 as usize] = '@';
                rp = (fr.0, fr.1);
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if psn[i][j] == 'O' {
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
