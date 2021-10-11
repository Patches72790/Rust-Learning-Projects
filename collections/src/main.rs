use collections::{make_spreadsheet, SpreadsheetCell, str_stuff, hashmap_me_bro, hashmap_from_vector, print_hashmap};
use collections::piglatinify;

fn main() {
    let spr = make_spreadsheet(); 
    for i in spr.iter() {
        match i {
            SpreadsheetCell::Int(num) => println!("A num: {}", num),
            SpreadsheetCell::Float(flt) => println!("A float: {}", flt),
            SpreadsheetCell::Text(txt) => println!("Some text: {}", txt),
        }
    }

    str_stuff();
    println!();

    let my_hm = hashmap_me_bro();
    for k in my_hm.keys() {
        println!("Key: {:?}, Val: {:?}", k, my_hm.get_key_value(&k[..]));
    }

    let v_hm = hashmap_from_vector(vec!["abc".to_string(), "xyz".to_string()], vec![123, 456]);

    print_hashmap(v_hm); 
    print_hashmap(my_hm);


    piglatinify("abcdef".to_string());
}
