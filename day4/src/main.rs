fn main() {
    println!("Hello, world!");

    

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    print_ip_address(home);
    print_ip_address(loopback);

    let ip_addr_1 = IpV4Addr {
        address: String::from("127.0.0.1"),
    };

    println!("{}", ip_addr_1.address);

    let m = Message::Write(String::from("hello!"));
    m.call();


    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    println!(sum);


}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IPAddrKind2 {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,   
}

struct IpV4Addr {
    address: String,
}

struct IPV6Addr {
    address: String,
}

enum IpAddrTwo {
    V4(IpV4Addr),
    V6(IPV6Addr),
}


fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}

fn print_ip_address(ip_kind_address: IpAddr) {
    println!("IP Kind : {:#?}, IP Address : {:#?}", ip_kind_address.kind, ip_kind_address.address);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("message called.");
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }

