#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let length1 = 50;

    println!(
        "The area_with_raw_nums of the rectangle is {} pixels.",
        area_with_raw_nums(width1, length1)
    );

    let rect1 = (40, 60);
    println!(
        "The area_with_tuple of the rectangle is {} pixels.",
        area_with_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    
    println!(
        "The area_with_struct of the rectangle {:?} is {} pixels.",
        rect2,
        area_with_struct(&rect2)
    );
    
    println!(
        "The rect.area of the rectangle {:?} is {} pixels.",
        rect2,
        rect2.area()
    );
}

fn area_with_raw_nums(width: u32, length: u32) -> u32 {
    width * length
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
