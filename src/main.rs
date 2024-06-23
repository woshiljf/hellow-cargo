// 入口函数

fn main() {
    let guess = 10;
    let age = get_name(guess);
    print!("{}", age)
}

fn get_name(age: i32) -> i32 {
    age + 20
}
