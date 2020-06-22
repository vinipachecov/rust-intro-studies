#[derive(Debug)]
enum IpdAddrKind {
    V4,
    V6
}

fn main () {
    let four = IpdAddrKind::V4;
    let six = IpdAddrKind::V6;
    route(four);
    route(six);

}

fn route (ip: IpdAddrKind) {
    println!("{:?}", ip);
}