#[derive(Debug)] //对这个结构体使用调试模式，这个注解实际上使用的是std::fmt::Debug
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::square(10);
    println!("{:?}", rect); //Rectangle { width: 10, height: 10 }
}
