use std::env;

mod test_format;

use test_format::TestFormat;

fn main() {
    env::args().next();

    let path = env::args().next().expect("usage: path");

    let t = TestFormat::parse(path);

    let r = lcs_rs::lcs(&t.s1, &t.s2);

    assert!(r == t.result_size);

    println!("all good!");
}
