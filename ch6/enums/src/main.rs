fn main(){
    let x = IpAddrKind::V4(String::from("Something"));
}

enum Option<IpAddrKind>{
    Some(IpAddrKind),
    None,
}

enum IpAddrKind {
    V4(String),
    V6(String),
}
