
#[allow(dead_code)] // これを追加
struct Color(i32, i32, i32);
#[allow(dead_code)] // これを追加
struct Point(i32, i32, i32); 
struct AlwaysEqual;

fn main() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;

    println!("Hello, world!");
}

