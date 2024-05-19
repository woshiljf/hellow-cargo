// use 使用其他的库进来
use std::io;

fn main() {
    println!("Hello, world!ffff");
    let mut guess = String::new();
    let msg = "猜测错误";
    io::stdin().read_line(&mut guess).expect(msg);
    // {}，相当于printf的%s，占据位置
    print!("你的猜测数是{}", guess)
}
