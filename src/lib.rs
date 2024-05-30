use std::{cmp::max, fmt::Debug};

#[cfg(test)]
mod test_format;

#[inline]
fn fill_matrice(s1: &str, s2: &str) -> Vec<Vec<usize>> {
    let mut mat: Vec<Vec<usize>> = vec![vec![0; s1.len() + 1]; s2.len() + 1];

    let mut s1_iter = s1.chars();
    let mut u = None;

    for i in 0..=s1.len() {
        let mut s2_iter = s2.chars();
        let mut v = None;

        for j in 0..=s2.len() {
            if i == 0 || j == 0 {
                mat[i][j] = 0;
            } else if u.unwrap() == v.unwrap() {
                mat[i][j] = mat[i - 1][j - 1] + 1;
            } else {
                mat[i][j] = max(mat[i - 1][j], mat[i][j - 1]);
            }
            v = s2_iter.next();
        }
        u = s1_iter.next();
    }
    mat
}

#[must_use]
pub fn lcs_len(s1: &str, s2: &str) -> usize {
    let mat = fill_matrice(s1, s2);
    mat[s1.len()][s2.len()]
}

#[must_use]
pub fn lcs(s1: &str, s2: &str) -> String {
    let mat = fill_matrice(s1, s2);

    let mut res = String::new();

    let mut s1_iter = s1.chars().rev().peekable();
    let mut s2_iter = s2.chars().rev().peekable();

    let mut i = s1.len();
    let mut j = s2.len();

    while i > 0 && j > 0 {
        if s1_iter.peek().unwrap() == s2_iter.peek().unwrap() {
            res.insert(0, *s1_iter.peek().unwrap());
            s1_iter.next();
            s2_iter.next();
            i -= 1;
            j -= 1;
        } else if mat[i - 1][j] > mat[i][j - 1] {
            s1_iter.next();
            i -= 1;
        } else {
            s2_iter.next();
            j -= 1;
        }
    }

    res
}

#[allow(dead_code)]
#[allow(clippy::needless_range_loop)]
fn print_mat<T: Debug>(mat: &[Vec<T>]) {
    for j in 0..mat[0].len() {
        for i in 0..mat.len() {
            print!("{:?} ", mat[i][j])
        }
        println!()
    }

    println!();
}

#[cfg(test)]
mod test {

    use crate::{lcs, lcs_len, test_format::TestFormat};

    #[test]
    fn run_tests() {
        for e in std::fs::read_dir("tests").expect("no test dir") {
            let entry = e.expect("err in entry");
            let path = entry.path();

            println!("testing {}", path.display());

            let test = TestFormat::parse(path);

            let res = lcs(&test.s1, &test.s2);
            assert!(test.result == res);

            let res = lcs_len(&test.s1, &test.s2);
            assert!(test.result_size == res);
        }
    }

    #[test]
    fn run_test() {
        let test = TestFormat::parse("tests/adn-10-10.test");

        let res = lcs(&test.s1, &test.s2);
        assert!(test.result == res);

        let res = lcs_len(&test.s1, &test.s2);
        assert!(test.result_size == res);
    }
}
