use matrices::Vector;

fn main() {
    let v1 = Vector::elements(&[1, 2, 3, 4]);
    let v2 = Vector::elements(&[4, 5, 6, 7]);

    let v3 = v1 * v2;

    println!("{v3}");
}
