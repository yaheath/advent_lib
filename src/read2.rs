use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin, StdinLock};
use std::iter;
use std::iter::Iterator;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Clone,Copy)]
pub enum ParseErr {
    Err,
    Skip,
}

pub trait SkippableErr {
    fn is_skipped(&self) -> bool;
}

impl SkippableErr for ParseErr {
    fn is_skipped(&self) -> bool {
        matches!(self, ParseErr::Skip)
    }
}

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
impl LineIter {
    pub fn new() -> LineIter {
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
}
impl Default for LineIter {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_line<T: FromStr>(s: &str) -> Option<T>
    where <T as FromStr>::Err: SkippableErr
{
    match s.trim_end_matches('\n').parse::<T>() {
        Ok(val) => Some(val),
        Err(e) => {
            if !e.is_skipped() {
                panic!("Invalid line: {}", s.trim());
            }
            None
        },
    }

}

pub fn input_from_iter<T: FromStr, I: Iterator<Item=String>>(line_iter: I) -> Vec<T>
    where <T as FromStr>::Err: SkippableErr
{
    line_iter
        .flat_map(|l| parse_line(&l))
        .collect()
}

pub fn input_as_string() -> String {
    LineIter::new()
        .chain(iter::once("".into()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn test_input<T: FromStr>(data: &str) -> Vec<T>
    where <T as FromStr>::Err: SkippableErr
{
    input_from_iter(data.lines().map(|l| l.into()))
}

pub fn read_input<T: FromStr>() -> Vec<T>
    where <T as FromStr>::Err: SkippableErr
{
    input_from_iter(LineIter::new())
}

pub fn grouped_input_from_iter<T: FromStr, I: Iterator<Item=String>>(line_iter: I) -> Vec<Vec<T>>
    where <T as FromStr>::Err: SkippableErr
{
    let mut data: Vec<Vec<T>> = Vec::new();
    let mut row: Vec<T> = Vec::new();
    for line in line_iter {
        let val = line.trim_end_matches('\n');
        if val.is_empty() {
            data.push(row);
            row = Vec::new();
        }
        else if let Some(val) = parse_line(val) {
            row.push(val);
        }
    };
    if !row.is_empty() {
        data.push(row);
    }
    data
}

pub fn read_grouped_input<T: FromStr>() -> Vec<Vec<T>>
    where <T as FromStr>::Err: SkippableErr
{
    grouped_input_from_iter(LineIter::new())
}

pub fn grouped_test_input<T: FromStr>(data: &str) -> Vec<Vec<T>>
    where <T as FromStr>::Err: SkippableErr
{
    grouped_input_from_iter(data.lines().map(|l| l.into()))
}

pub fn sectioned_input_from_iter<T1: FromStr, T2: FromStr, I: Iterator<Item=String>>(mut line_iter: I) -> (Vec<T1>,Vec<T2>)
    where <T1 as FromStr>::Err: SkippableErr,
          <T2 as FromStr>::Err: SkippableErr
{
    let mut data1: Vec<T1> = Vec::new();
    let mut data2: Vec<T2> = Vec::new();

    for l in line_iter.by_ref() {
        let l = l.trim_end_matches('\n');
        if l.is_empty() { break; }
        if let Some(val) = parse_line(l) {
            data1.push(val);
        }
    }
    for l in line_iter {
        let l = l.trim_end_matches('\n');
        if let Some(val) = parse_line(l) {
            data2.push(val);
        }
    }
    (data1, data2)
}

pub fn read_sectioned_input<T1: FromStr, T2: FromStr>() -> (Vec<T1>, Vec<T2>)
    where <T1 as FromStr>::Err: SkippableErr,
          <T2 as FromStr>::Err: SkippableErr
{
    sectioned_input_from_iter(LineIter::new())
}

pub fn sectioned_test_input<T1: FromStr, T2: FromStr>(data: &str) -> (Vec<T1>, Vec<T2>)
    where <T1 as FromStr>::Err: SkippableErr,
          <T2 as FromStr>::Err: SkippableErr
{
    sectioned_input_from_iter(data.lines().map(|l| l.into()))
}
