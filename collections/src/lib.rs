mod matrices;

pub use matrices::{multiply_matrix, print_matrix, vector_matrix_multiply};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_multiply_1() {
        let m1 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let m2 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let m3 = multiply_matrix(m1.clone(), m2.clone());
        assert_eq!(m3, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    }

    #[test]
    fn test_matrix_multiply_2() {
        let m1 = vec![vec![1, 2, 3]];
        let m2 = vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]];

        assert_eq!(
            multiply_matrix(m1.clone(), m2.clone()),
            vec![vec![22, 28, 34]]
        );
    }

    #[test]
    fn test_vector_multiply_1() {
        let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let v = vec![1, 0, 0];
        let res = vector_matrix_multiply(v.clone(), m.clone());

        assert_eq!(res, vec![1, 2, 3]);
    }
}
