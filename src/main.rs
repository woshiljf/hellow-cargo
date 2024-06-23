
// 入口函数

use std::string;

fn main() {
    let mut guess = String::from("我是大海沟");
    get_name(&mut guess);
    print!("{}", guess)
}

fn get_name(name: &mut String) {

    print!("{}", name);
    // name 只是借用，所有没有权利修改;
    // name.push_str("测试一下");

    // 可以修改
    name.push_str("测试一下");   
    name.len();
}

