use std::io::{prelude::BufRead, prelude::Write, stdin, stdout, BufWriter};
use std::str;

/*
    Author: Koushik Sahu
    Created: 08:20:04 PM(20:20:04) IST(+05:30) 17-12-2024 Tue
*/

fn combo(operand: i32, reg: &(i32, i32, i32)) -> i32 {
    match operand {
        0..=3 => return operand,
        4 => return reg.0,
        5 => return reg.1,
        6 => return reg.2,
        7 => panic!("7 is reserved and should not appear in valid program"),
        _ => panic!(">3 bit number should not appear in valid program"),
    }
}

// returns (instructor pointer, output)
fn execute(i: &mut usize, prog: &Vec<i32>, reg: &mut (i32, i32, i32)) -> i32 {
    let operator = prog[*i];
    let operand = prog[*i + 1];
    let mut output = -1;
    match operator {
        0 => {
            let nr = reg.0;
            let dr: i32 = 2i32.pow(combo(operand, reg) as u32);
            reg.0 = nr / dr;
            *i += 2;
        }
        1 => {
            let res = reg.1 ^ operand;
            reg.1 = res;
            *i += 2;
        }
        2 => {
            let res = combo(operand, reg) % 8;
            reg.1 = res;
            *i += 2;
        }
        3 => {
            if reg.0 != 0 {
                *i = operand as usize;
            } else {
                *i += 2;
            }
        }
        4 => {
            let res = reg.1 ^ reg.2;
            reg.1 = res;
            *i += 2;
        }
        5 => {
            let res = combo(operand, reg) % 8;
            output = res;
            *i += 2;
        }
        6 => {
            let nr = reg.0;
            let dr: i32 = 2i32.pow(combo(operand, reg) as u32);
            reg.1 = nr / dr;
            *i += 2;
        }
        7 => {
            let nr = reg.0;
            let dr: i32 = 2i32.pow(combo(operand, reg) as u32);
            reg.2 = nr / dr;
            *i += 2;
        }
        _ => {
            panic!(">3 bit operator should not appear in valid programs")
        }
    }
    output
}

fn solve<R: BufRead, W: Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let mut reg: (i32, i32, i32) = (-1, -1, -1);
    let mut prog = Vec::<i32>::new();
    while let Some(s) = sc.try_next() {
        if s.starts_with("Register") {
            let rs = s.split_whitespace().collect::<Vec<&str>>();
            match rs[1] {
                "A:" => reg.0 = rs[2].parse::<i32>().unwrap(),
                "B:" => reg.1 = rs[2].parse::<i32>().unwrap(),
                "C:" => reg.2 = rs[2].parse::<i32>().unwrap(),
                _ => (),
            }
        } else if s.starts_with("Program") {
            let ps = s.split_whitespace().collect::<Vec<&str>>();
            let p = ps[1]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            prog = p;
        }
    }
    let mut i = 0;
    let mut ans = String::new();
    while i < prog.len() {
        let output = execute(&mut i, &prog, &mut reg);
        if output != -1 {
            ans = format!("{ans}{output},");
        }
    }
    ans = ans.trim_matches(',').to_owned();
    writeln!(wr, "{ans}").unwrap();
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
