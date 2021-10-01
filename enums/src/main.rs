
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));


    let some_num = Some(5);

    let some_num: Option<i32> = match some_num {
        Some(i) => Some(i * i),
        None => None,
    };

    println!("{:?}", some_num);
    
}
