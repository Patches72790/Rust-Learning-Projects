use collections::{multiply_matrix, print_matrix};

fn main() {
    let m1 = vec![vec![1, 0], vec![0, 1]];
    let m2 = vec![vec![1, 2], vec![3, 4]];

    let m3 = multiply_matrix(m1.clone(), m2.clone());

    print_matrix(m1);
    print_matrix(m2);
    print_matrix(m3);
}
