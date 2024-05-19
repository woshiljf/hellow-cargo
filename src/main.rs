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
        // let guess: u32 = guess.trim().parse().expect(msg);
    

        // 提升代码的健壮性，如果解析失败，让用户继续猜测
        // 匹配，如果正确的话，会返回一个Ok，否则返回一个Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
         };


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
