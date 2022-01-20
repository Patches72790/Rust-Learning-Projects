
enum IpAddr {
    V4(String),
    V6(String),
}

struct SocketAddr {
    ip: IpAddr,
    port: usize,
}

impl SocketAddr {
    pub fn new(ip: IpAddr, port: usize) -> SocketAddr {
        SocketAddr { ip, port }
    }

    pub fn ip_addr_type(&self) {
        match &self.ip {
            IpAddr::V4(ip) => println!("This is an IPv4: {}", ip),
            IpAddr::V6(ip) => println!("This is an IPv6: {}", ip),
        }
    }
}

pub fn ip_stuff() {
    let my_addr = SocketAddr::new(IpAddr::V4(String::from("127.0.0.1")), 8080);
    let v6_addr = SocketAddr::new(
        IpAddr::V6(String::from("2001:abcd:4269:0123:af31:0000:9812:ffff")),
        5678,
    );

    my_addr.ip_addr_type();
    v6_addr.ip_addr_type();
}
