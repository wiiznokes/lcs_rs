use std::{cmp::max, fmt::Debug};

pub fn lcs(s1: &str, s2: &str) -> usize {

    let mut mat: Vec<Vec<i32>> = vec![vec![0; s1.len() + 1]; s2.len() + 1];

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


    let mut s1_iter = s1.char_indices().rev().peekable();
    let mut s2_iter = s1.char_indices().rev().peekable();

    let mut res = 0;

    print_mat(&mat);
    

    while let (Some((i, u)), Some((j, v))) = (s1_iter.peek(), s2_iter.peek()) {

        if u == v {
            res += 1;
            s1_iter.next();
            s2_iter.next();
        } else if mat[i - 1][*j] > mat[*i][j - 1] {
            s1_iter.next();
        } else {
            s2_iter.next();
        }
    }

    println!("res = {res}");
    res
}


fn print_mat<T: Debug>(mat: &Vec<Vec<T>>) {
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
    use crate::lcs;

    #[test]
    fn run_test() {
        assert!(lcs("GCACAGCGGT", "TTGTGAAATC") == 4);
    }

}