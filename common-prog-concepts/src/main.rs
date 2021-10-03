use std::io;

const NUM: i32 = 100_000;
const MONTHS: [&str; 4] = ["January", "February", "March", "April"];

fn scalar_types() {
    let byte = b'A';
    let x: u8 = 255;
    let y: u8 = 255;
    let z: u8 = 255;

    println!("Isize: {}", x);
    println!("Byte: {}", byte);
    println!("Too big, so overflow! {}", x.wrapping_add(1)); // compile with --release flag to allow for wrapping
    
    // use checked or wrapping methods to carefully watch for overflow
    match y.checked_add(1) {
        None => println!("Overflow!"),
        Some(_) => println!("Success: {}", y)
    }

    // overflowing methods
    let z_overflowed = z.overflowing_add(1);
    println!("Z overflowed ? {:?}", z_overflowed);

}

fn char_types() -> char {
    return '😻';
}

fn redeclaration() {
    let x = NUM;
    let x = x * 2;
    let x = x * 2;

    println!("Here is x: {}", x);
    println!("Num of spaces: {}", "    ".len());

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces: {}", spaces);

}

fn compound_types() {
    let tuple: (i32, f64, u8) = (500, 3.14, 255);
    println!("Here is a tuple: {:?}", tuple);

    // dot operator to access nth element of tuple
    println!("Here is the 2nd element of tuple: {}", tuple.1);

    // destructuring
    let (x, y, z) = tuple;
    println!("Pattern matching destructuring: {} {} {}", x, y, z);
}

fn main() {
    redeclaration();
    scalar_types();
    println!("Char type: {}", char_types());
    compound_types();
    array_types();
    prevent_invalid_access();
}

fn prevent_invalid_access() {
    let a = [1, 2, 3, 4, 5];
    println!("Enter an array index:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);
}

fn array_types() {
    let a = [1, 2, 3, 4, 5];
    println!("Here is an array! {:?}", a);

    println!("Here are the months: {:?}", MONTHS);

    let a2: [i32; 5] = [6, 7, 8, 9, 10];
    println!("An array with type annotation: {:?}", a2);

    let a3 = [0; 5];
    println!("Here is an array initialized to 0: {:?}", a3);
}

