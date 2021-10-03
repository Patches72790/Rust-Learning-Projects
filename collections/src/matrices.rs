pub fn multiply_matrix(m1: Vec<Vec<i32>>, m2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut multiplied: Vec<Vec<i32>> = vec![vec![0; m1.len()]; m1.len()];
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < m1.len() {
        while j < m1.len() {
            while k < m1[j].len() {
                multiplied[i][j] = m1[i][j] * m2[j][k];
                k = k + 1;
            }
            k = 0;
            j = j + 1;
        }
        j = 0;
        i = i + 1;
    }

    return multiplied;
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
