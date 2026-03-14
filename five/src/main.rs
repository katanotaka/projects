fn main() {
    let mut x = five();
    println!("Hello, {x} world!");

    x = plus_one(x);
    println!("hello, {x} world");
}

fn five() -> i32 {
    5
}

fn plus_one(y: i32) -> i32 {
    y + 700
}

