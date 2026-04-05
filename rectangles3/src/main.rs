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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 構造体に「関連関数（Associated Functions）」という特別な役割を持たせ
    // 長方形（Rectangle）という設計図を使って、正方形（Square）を簡単に作る
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
 
    // 長方形の幅と高さを定義   
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: dbg!(10 * 2),
        height: dbg!(1 * 40),
    };

    // 長方形は非ゼロの幅を持っていますか？
    if rect1.width() {
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    }

    // 正方形を定義
    let sq1 = Rectangle::square(100);
    println!("Square area : {}", sq1.area());
}
