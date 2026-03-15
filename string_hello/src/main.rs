fn main() {
    let mut s = String::from("HELLO"); //ヒープを確保

    s.push_str(",world!!");

    println!("{s}");
}   //スコープはおしまい。メモリは返還される
