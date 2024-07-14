/**!SECTION
 * 学习结构体
 */
#[derive(Debug)]
pub struct Rectangle { 
    width: u32,
    height: u32,

}
// 给结构体定义方法
impl Rectangle {
    /**
     * 关联函数不是方法，关联函数的第一个参数不是self，也就是说不是结构体本身
     */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn other_area(&self, name: String) -> String {  

        name
    }
    
}
// 使用struct 定义一个长方形，并计算它的面积
pub fn enter_struct() { 
    let rect: Rectangle = Rectangle { 
        width: 30,
        height: 50,
    };

    let name = Rectangle::other_area(&rect, "longddahai".to_string());

    println!("rect: {:?}", area(&rect));
    print!("rect: {:#?}", rect);
    print!("rect: {:#?}", rect.area());
    print!("rect: {:#?}", name)
}

fn area(rect: &Rectangle) -> u32 { 
    rect.width * rect.height
}