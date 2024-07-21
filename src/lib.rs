
// 定义一个模块module   
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }

}

// rust 的 path，可以使用绝对路径从一个crate 开始，或者使用相对路径从当前模块开始
pub  fn eat_at_restaurant() { 
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();


}