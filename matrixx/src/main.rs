use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
        // 何番にアクセスするか指定

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 読み込みに失敗

    let index: usize = index
        .trim()
        .parse()
        .expect("Index was not a number");
               // 値は数字ではありません

    let element = a[index];

    println!("The value at index {index} is: {element}");
           // {index}番目の値は{element}です
}
