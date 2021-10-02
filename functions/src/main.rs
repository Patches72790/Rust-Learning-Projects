
fn another_function(x: i32) -> i32 {
    let x = x;
    println!("The value of x is {}", x);    
    x * x
}

fn loops() {
    let a = [1, 2, 3, 4, 5];

    for i in a.iter() {
        print!("{}, ", i);
    }
}

fn plus_one_or_two(x: i32) -> i32 {
    if x > 10 {
        return x + 1;
    }
    x + 2
}

fn return_five() -> i32 {
    5
}
/**
 * Documentation comment
 */
fn main() {
    let _x = another_function(25);

    let y = {
        let mut x = 3;
        x = x + 1;
        x * x
    };

    println!("Result: {}", y); 
    println!("This is called with function that returns five: {}", return_five());
    println!("Return 5 and add one: {}", plus_one_or_two(return_five()));
    loops();
}
