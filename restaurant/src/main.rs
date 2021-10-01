use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let mut map = HashMap::new();
 
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    map.insert(1, 42);

    let x = map.get(&1);

    match x {
        Some(x) => println!("{}", x),
        None => () 
    }
}
