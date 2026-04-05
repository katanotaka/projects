#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }  

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
 
    // 長方形の幅と高さを定義   
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect2);

    // 長方形は非ゼロの幅を持っていますか？
    if rect2.width() {
        println!(
            "The area of the rectangle is {} square pixels.",
            rect2.area()
        );
    }
}
