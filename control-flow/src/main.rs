fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn c_to_f(temp: f32) -> f32 {
    (temp * 9.0 / 5.0) + 32.0
}


fn for_loop_me(array: [i32; 5]) {
    let mut count = 0;
    for item in array.iter() {
        count = count + 1;
        print!("{}", item);
        if count != array.len() {
            print!(",");
        }
    }
}


fn loop_me(num: i32) -> i32 {
    let mut count = 0;
    loop {
        if count == num {
            println!("Done!");
            break 42;
        } else {
            print!(".");
            count = count + 1;
        }
    }
}

fn conditional(condition: bool) {
    let number = if condition { "true" } else { "false" };
    println!("{}", number);
}


fn main() {
    conditional(true);
    conditional(false);
    loop_me(10);

    let a = [1, 2, 3, 4, 5];
    for_loop_me(a);

    println!("Temp: {}", c_to_f(f_to_c(212.0)));
}
