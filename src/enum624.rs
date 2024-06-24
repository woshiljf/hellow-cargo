enum IpAddrKind { 
    V4,
    V6
}
// 定义一个结构体，使用枚举 作为类型
struct IpAddr { 
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum Message { 
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

pub fn testEnum() { 

   let ip =  route(IpAddrKind::V4);
   let m = Message::Write(String::from("hello"));
   let m = Message::ChangeColor((0), (244), (233));
   print!("{:?}---{:#?}", Message::Quit, m);
 }

 fn route(ip_kind: IpAddrKind) -> IpAddrKind { 

    ip_kind

 }