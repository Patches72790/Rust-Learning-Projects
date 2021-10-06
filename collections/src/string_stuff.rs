pub fn str_stuff() {
    let v = vec![String::from("hello")];

    let g: String = match v.get(0) {
        Some(val) => { 
            println!("{}", val);
            val.to_string()
        },
        None => {
            println!("No string here");
            "".to_string()
        },
    };

    let optional = v.get(0);
    if let Some(i) = optional {
        println!("{}", i);
    }

    let optional = match optional {
        Some(val) => val,
        _ => "",
    };

    let g2 = g + "abcdefg" + &optional.to_string();
    println!("{}", g2);    

    // strings in utf-8 are not 1-1 mapping with index, so String by default is NOT indexable!
    let hello = "Здравствуйте";
    print!("Letters in hello in Russian: ");
    for c in hello.chars() {
        print!("{} ", c);
    }

    print!("\nBytes in previous word: ");
    for b in hello.bytes() {
        print!("{} ", b);
    }
}

