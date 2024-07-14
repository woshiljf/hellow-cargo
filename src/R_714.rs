
struct IpAddr { 

    address: String
}

struct user { 
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool
}

fn entry() { 


    let rect = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area1(rect.0, rect.1))

}

fn area2(width: u32, height: u32) -> u32 { 

    width * height
}

fn area1(dim: (u32, u32)) -> u32 { 

    dim.0 * dim.1
}