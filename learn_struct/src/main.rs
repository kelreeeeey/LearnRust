#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // reptiles call it `normal method` (in python)
    fn area(&self) -> u32 { 
        self.width * self.height
    }

    // reptiles call it `static method`(in python)
    // crustacean call it `associative function(?)`
    fn square(size: u32) -> Rectangle { 
        Rectangle {width: size, height: size}
    }
}



fn main() {
    let rect = Rectangle {
        width: 50,
        height: 30,
    };
    let square = Rectangle::square(25);
    println!(
        "The rectangle is \n{:?},\n with area of {}",
        rect,
        rect.area()
    );
    println!(
        "The square is \n{:?},\n with area of {}",
        square,
        square.area()
    )
}
