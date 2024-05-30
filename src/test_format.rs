use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct TestFormat {
    pub s1: String,
    pub s2: String,
    pub result_size: usize,
    pub result: String,
}

fn decode_utf8(buf: Vec<u8>) -> String {
    match String::from_utf8(buf) {
        Ok(lines) => lines,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).to_string(),
    }
}

impl TestFormat {
    pub fn parse<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path).expect("no file");

        let mut content = Vec::new();
        file.read_to_end(&mut content).expect("can't read");

        let s = decode_utf8(content);

        let mut lines = s.lines();

        let s1 = lines.next().expect("no s1");
        let s2 = lines.next().expect("no s2");
        let result_size = lines.next().expect("no res size");
        let result_size = result_size.parse::<usize>().expect("no res size number");
        let result = lines.next().expect("no res");

        Self {
            s1: s1.to_string(),
            s2: s2.to_string(),
            result_size,
            result: result.to_string(),
        }
    }
}
