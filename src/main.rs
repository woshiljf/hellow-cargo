use std::io;

fn main() {
    println!("Hello, world!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("错误了");
    print!("你的猜测数是{}", guess)
}
