pub fn multiply_matrix(m1: Vec<Vec<i32>>, m2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if m1[0].len() != m2.len() {
        panic!("Matrix 1 columns must be equal to matrix 2 rows!");
    }

    let mut multiplied: Vec<Vec<i32>> = vec![vec![0; m2[0].len()]; m1.len()];

    for i in 0..m1.len() {
        for j in 0..m1[0].len() {
            for k in 0..m2.len() {
                multiplied[i][j] += m1[i][k] * m2[k][j];
            }
        }
    }

    return multiplied;
}

pub fn vector_matrix_multiply(v: Vec<i32>, m: Vec<Vec<i32>>) -> Vec<i32> {
    if v.len() != m[0].len() {
        panic!("Vector rows must equal matrix columns!");
    }

    let mut multiplied: Vec<i32> = vec![0; v.len()];

    for i in 0..v.len() {
        for j in 0..m[0].len() {
            multiplied[i] += v[j] * m[j][i];
        }
    }

    multiplied
}

pub fn print_matrix(m: Vec<Vec<i32>>) {
    for row in m.iter() {
        print!("|");
        for col in row.iter() {
            print!("{}", col);
        }
        println!("|");
    }
    println!();
}

