use std::env;

mod test_format;

use test_format::TestFormat;

fn main() {
    let mut args = env::args();

    args.next();

    let path = args.next().expect("usage: path");

    let t = TestFormat::parse(path);

    let now = std::time::Instant::now();

    let r = lcs_rs::lcs(&t.s1, &t.s2);

    let elapsed = now.elapsed();
    println!("Time: {:?}", elapsed);

    assert!(r == t.result);
}
