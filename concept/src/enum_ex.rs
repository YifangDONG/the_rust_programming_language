#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn print(ip: IpAddr) {
        match ip {
            IpAddr::V4(a, b, c, d) => println!("ip: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(address) => println!("ip: {}", address),
        }
    }
}

/*
Option is in prelude https://doc.rust-lang.org/std/prelude/index.html
enum Option<T> {
    Some(T),
    None,
}
 */

#[test]
fn create_ip_address() {
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("::1"));
    println!("{:?}", ipv4);
    println!("{:?}", ipv6);
}

#[test]
fn pattern_matching() {
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("::1"));
    IpAddr::print(ipv4);
    IpAddr::print(ipv6);
}

#[test]
fn if_let() {
    let x = Some(0);

    if let Some(3) = x {
        println!("three");
    } else {
        println!("Ohters");
    }
}
