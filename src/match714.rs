enum Coin { 
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[derive(Debug)]
enum UsState { 

    Alabama,
    Alaska,
    // ... etc
}

// 绑定值模式
enum Coin1 {
    Penny,
    Nickel,
    Dime,
    // 绑定值模式，绑定值模式可以绑定一个值
    Quarter(UsState),
}


// match 表达式
fn value_in_cents(coin: Coin1) -> u8 { 

    // match 表达式,match 表达式会返回一个值，match 表达式会对每一个分支进行求值，并返回第一个匹配的分支的值
    match coin {
        Coin1::Penny => {

            1
        },
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter(state) => {

            print!("{:#?}", state);
            print!("{:#?}", UsState::Alabama);
            25

        },
    }

}

pub fn mainTest1() {

    // let coin = Coin::Penny;
    let coin = Coin1::Quarter(UsState::Alabama);
    let value = value_in_cents(coin);
    println!("{}", value);

}