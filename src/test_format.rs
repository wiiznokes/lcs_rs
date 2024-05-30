use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct TestFormat {
    pub s1: String,
    pub s2: String,
    pub result_size: usize,
    pub _result: String,
}

impl TestFormat {
    pub fn parse<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let file = File::open(path).expect("no file");

        let mut lines = io::BufReader::new(file).lines();

        let s1 = lines.next().expect("no s1").expect("no s1");
        let s2 = lines.next().expect("no s2").expect("no s2");
        let result_size = lines.next().expect("no res size").expect("no res size");
        let result_size = result_size.parse::<usize>().expect("no res size number");
        let result = lines.next().expect("no res").expect("no res");

        Self {
            s1,
            s2,
            result_size,
            _result: result,
        }
    }
}
