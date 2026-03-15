fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    //'{}'の長さは、{}です
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します
    (s, length)
}