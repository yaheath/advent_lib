use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin, StdinLock};
use std::iter;
use std::iter::Iterator;
use std::str::FromStr;
use std::vec::Vec;

lazy_static! {
    static ref STDIN: Stdin = io::stdin();
}

fn stdinlock() -> StdinLock<'static> {
    STDIN.lock()
}

enum LineIters {
    File(Lines<BufReader<File>>),
    Stdin(Lines<StdinLock<'static>>),
}

pub struct LineIter {
    inner: LineIters,
}
impl Iterator for LineIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            LineIters::File(i) => i.next().map(|x| x.unwrap()),
            LineIters::Stdin(i) => i.next().map(|x| x.unwrap()),
        }
    }
}

pub fn input_lines() -> LineIter {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        LineIter {
            inner: LineIters::File(
                BufReader::new(file).lines()
            ),
        }
    } else {
        let lock = stdinlock();
        LineIter {
            inner: LineIters::Stdin(lock.lines()),
        }
    }
}

pub fn input_from_iter<T: FromStr, I: Iterator<Item=String>>(line_iter: I) -> Vec<T> {
    let mut data: Vec<T> = Vec::new();
    for line in line_iter {
        match line.trim_end().parse::<T>() {
            Ok(val) => data.push(val),
            Err(_) => eprintln!("Invalid line: {}", line.trim()),
        }
    }
    data
}

pub fn input_as_string() -> String {
    input_lines()
        .chain(iter::once("".into()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn test_input<T: FromStr>(data: &str) -> Vec<T> {
    input_from_iter(data.lines().map(|l| l.into()))
}

pub fn read_input<T: FromStr>() -> Vec<T> {
    input_from_iter(input_lines())
}

pub fn grouped_input_from_iter<T: FromStr, I: Iterator<Item=String>>(line_iter: I) -> Vec<Vec<T>> {
    let mut data: Vec<Vec<T>> = Vec::new();
    let mut row: Vec<T> = Vec::new();
    for line in line_iter {
        let val = line.trim_end();
        if val.is_empty() {
            data.push(row);
            row = Vec::new();
        }
        else {
            match val.parse::<T>() {
                Ok(val) => row.push(val),
                Err(_) => eprintln!("Invalid line: {}", val),
            }
        }
    };
    if !row.is_empty() {
        data.push(row);
    }
    data
}

pub fn read_grouped_input<T: FromStr>() -> Vec<Vec<T>> {
    grouped_input_from_iter(input_lines())
}

pub fn grouped_test_input<T: FromStr>(data: &str) -> Vec<Vec<T>> {
    grouped_input_from_iter(data.lines().map(|l| l.into()))
}

pub fn sectioned_input_from_iter<T1: FromStr, T2: FromStr, I: Iterator<Item=String>>(mut line_iter: I) -> (Vec<T1>,Vec<T2>) {
    let mut data1: Vec<T1> = Vec::new();
    let mut data2: Vec<T2> = Vec::new();

    for l in line_iter.by_ref() {
        let l = l.trim_end();
        if l.is_empty() { break; }
        match l.parse::<T1>() {
            Ok(val) => data1.push(val),
            Err(_) => eprintln!("Invalid line: {}", l),
        }
    }
    for l in line_iter {
        let l = l.trim_end();
        match l.parse::<T2>() {
            Ok(val) => data2.push(val),
            Err(_) => eprintln!("Invalid line: {}", l),
        }
    }
    (data1, data2)
}

pub fn read_sectioned_input<T1: FromStr, T2: FromStr>() -> (Vec<T1>, Vec<T2>) {
    sectioned_input_from_iter(input_lines())
}

pub fn sectioned_test_input<T1: FromStr, T2: FromStr>(data: &str) -> (Vec<T1>, Vec<T2>) {
    sectioned_input_from_iter(data.lines().map(|l| l.into()))
}
