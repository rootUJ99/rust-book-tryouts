#[derive(Debug)]
enum IpAddr { 
    Ipv4(String),
    Ipv6(String),
    Unknown,
}

impl IpAddr {
    fn check_if_valid_ip(&self){
        match *self {
            IpAddr::Ipv4(_) => println!("ip v4 it is"),
            IpAddr::Ipv6(_) => println!("ip v6 it is"),
            IpAddr::Unknown => println!("it is unknown"),
        }
    }
}

fn main() {
    let ipv4 = IpAddr::Ipv4(String::from("183.11.20.01"));
    let ipv6 = IpAddr::Ipv6(String::from("1050:0:0:0:5:600:300c:326b"));
    let unknown = IpAddr::Unknown;

    println!("{:?} {:?} {:?}", ipv4, ipv6, unknown);
    ipv4.check_if_valid_ip();
    ipv6.check_if_valid_ip();
    unknown.check_if_valid_ip();

}
