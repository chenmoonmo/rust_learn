struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Rectangle {
        Rectangle { width, height }
    }
    fn get_area(&self) -> i32 {
        self.width * self.height / 2
    }
    fn width(&self) -> i32 {
        self.width
    }
}

pub fn about_impl() {
    let square = Rectangle::new(10, 10);
    let w = square.width();
    let area = square.get_area();

    println!("width of square is {}, area of square is {}",w,area)
}
