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
// 给枚举实现方法
impl Message { 
    fn call(&self) {
        println!("{:?}", self);
    }
    
}
pub fn testEnum() { 

   let ip =  route(IpAddrKind::V4);
   let w = Message::Write(String::from("hello"));
   let m = Message::ChangeColor((0), (244), (233));
   print!("{:?}---{:#?}", Message::Quit, m);

   let x: i8 = 8;
//    let y: Option<i32> = Some(5);
   let y= 5;

   let result = x + y;
    
   w.call();
   m.call();

 }

 fn route(ip_kind: IpAddrKind) -> IpAddrKind { 

    ip_kind

 }