/**!SECTION
 * 学习结构体
 */

pub struct User {
    pub username: String,
    pub age: u32,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

// tuple struct
pub struct Color(i32, i32, i32);

pub fn lean_struct() -> User {
    let mut user1 = User {
        email: String::from("longdahaiEMAIL"),
        username: String::from("someusername123"),
        age: 20,
        sign_in_count: 1,
        active: true,
    };

    let mut user2: User = User {
        email: String::from("1"),
        username: String::from("2"),
        // 新的赋值
        ..user1
    };

    // 元组结构体的实例
    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

    user1
}
/**
 * 函数式声明，构造一个user
 */
fn build_user(email: String, username: String) -> User {
    let mut user = User {
        email,
        username,
        age: 20,
        sign_in_count: 1,
        active: true,
    };

    user
}
