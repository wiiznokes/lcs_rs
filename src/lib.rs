use std::cmp::max;

pub fn lcs(s1: &str, s2: &str) -> usize {

    let mut mat: Vec<Vec<i32>> = vec![vec![0; s1.len() + 1]; s2.len() + 1];

    for (i, u) in s1.char_indices() {
        for (j, v) in s2.char_indices() {
            if i == 0 || j == 0 {
                mat[i][j] = 0;
            } else if u == v {
                mat[i][j] = mat[i - 1][j - 1] + 1;
            } else {
                mat[i][j] = max(mat[i - 1][j], mat[i][j - 1]);
            }
        }
    }

    let mut s1_iter = s1.char_indices().rev().peekable();
    let mut s2_iter = s1.char_indices().rev().peekable();

    let mut res = 0;

    for v in &mat {
        println!("{:?}", v);
    }

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


#[cfg(test)]
mod test {
    use crate::lcs;

    #[test]
    fn run_test() {
        assert!(lcs("GCACAGCGGT", "TTGTGAAATC") == 4);
    }

}