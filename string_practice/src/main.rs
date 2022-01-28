fn my_func() {
    println!("Func 1!");
}

fn my_func2() {
    println!("Func 2!");
}

static FN_ARRAY: [fn(); 2] = [my_func, my_func2];

fn main() {
    let strs = vec!["Hello", "World"];
    let strs_u8 = strs.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();

    strs_u8.iter().for_each(|s| {
        println!(
            "String: {}\nBytes: {:?}",
            String::from_utf8(s.to_vec()).unwrap(),
            s
        );
    });

    FN_ARRAY.iter().for_each(|f| f());
}
