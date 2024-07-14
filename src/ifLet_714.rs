// if let 和 match 的区别

fn main() {


    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }


    // match 表达式 和 if let 表达式的区别
    match some_u8_value {
        3 => println!("three"),
    }


    if let Some(3) = some_u8_value {
        println!("three");
    }

}