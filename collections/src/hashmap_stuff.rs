use std::collections::HashMap;

pub fn hashmap_me_bro() -> HashMap<String, i32> {
    let mut hm = HashMap::new();

    hm.insert("abc".to_string(), 123);
    hm.insert("xyz".to_string(), 789);

    hm
}


pub fn hashmap_from_vector(key_vector: Vec<String>, value_vector: Vec<i32>) -> HashMap<String, i32> {
    key_vector.into_iter().zip(value_vector.into_iter()).collect()
}


pub fn print_hashmap<K: std::fmt::Display, V: std::fmt::Display>(hm: HashMap<K, V>) {
    for (k, v) in &hm {
        println!("Key: {}, Val: {}", k, v);
    }
}
