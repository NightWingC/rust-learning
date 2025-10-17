enum IpAddrkind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrkind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrkind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrkind::v6,
        address: String::from("::1"),
    };

}