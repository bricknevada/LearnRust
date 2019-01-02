#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    }
    fn rectangle(height: u32, width: u32) -> Rectangle {
        Rectangle{
            height: height,
            width: width
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_height(&mut self){
        self.height = 1000;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        let mut return_value : bool = false;
        return_value = self.width > other.width && self.height > self.width;
        return_value
    }
}

fn main() {
    let mut rect1 = Rectangle{width: 30, height: 50};
    println!("rect1 is {:#?}", rect1);
    println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
    );
    let mut rect2 = Rectangle{width: 15, height: 25};
    println!("will it fit? {}", rect1.can_hold(&rect2));

    let my_rectangle : Rectangle = Rectangle::square(10);
    let other_rectangle : Rectangle = Rectangle::rectangle(30, 100);
}
