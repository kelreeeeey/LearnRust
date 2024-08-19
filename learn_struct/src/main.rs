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
    let rect = Rectangle {
        width: 50,
        height: 30,
    };
    println!(
        "The rectangle is \n{:?},\n with area of {}",
        rect,
        rect.area()
    )
}
