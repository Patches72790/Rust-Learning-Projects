fn find_first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("i: {}, &item: {}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn borrows(some_string: String) {
    println!("Borrowed in function: {}", some_string);
}

fn main() {
    let s = String::from("Hello");

    println!("{}", s);

    // s borrowed here and no longer usable
    let x = s.clone();
    println!("{}", x);

    borrows(x.clone());
    println!("{}", (x));

    let new_str = create_str("What a world".to_string());
    println!("{}", new_str);

    let mut my_ref = String::from("I'm a mutable ref string");
    take_a_ref(&mut my_ref);
    println!("{}", my_ref);

    // mutable string refs only allowed once
    let mut s = String::from("Borrowing mutable string refs is only allowed once");
    let r1 = &mut s;
    // can't borrow mutable and immutable simultaneously
    //    let r3 = &s;
    //    let r2 = &mut s;

    println!("{}", r1 /* r2 */);

    let s = String::from("Hello world!");
    let word = find_first_word(&s);
    let another = find_first_word("here is another phrase");
    println!("{}, {}", word, another);
}

fn create_str(val: String) -> String {
    String::from(val)
}

fn take_a_ref(val: &mut String) {
    println!("{}", *val);
    (*val).push_str(", and now I've added some stuff too");
}
