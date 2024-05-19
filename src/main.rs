// use 使用其他的库进来
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!ffff");
    println!("请输入你的猜测数");   

    let msg = "猜测错误";
    let secret_number  = rand::thread_rng().gen_range(1..101);
    println!("随机数是{}", secret_number);  
    loop {
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect(msg);
    
    
        // 可以重复声明同名的变量，会覆盖上一个变量
        let guess: u32 = guess.trim().parse().expect(msg);
    
        match guess.cmp(&secret_number) {
    
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
            
        }
    
    }


}
