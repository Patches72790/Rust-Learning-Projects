fn borrow_a_string(s: &str) {
    println!("Hey, I borrowed this string: {}", s);
}

fn own_a_string(moved_str: String) {
    println!("Now I owwwwwn this string!!! {}", moved_str);
}

fn add_some_stuff_to_a_string(s: &mut String, stuff: &str) {
    s.push_str(stuff);
}

pub fn str_stuff() {
    let mut my_string = String::from("Calcifer the great and powerful fire daemon");
    borrow_a_string(&my_string);
    own_a_string(my_string.clone());
    //    borrow_a_string(&my_string);
    add_some_stuff_to_a_string(&mut my_string, " ----> Here's some stuff <----");
    borrow_a_string(&my_string);
}
