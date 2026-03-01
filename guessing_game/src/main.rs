use std::io;
use rand::Rng;
fn main() {
    println!("Guess the Number");
    
    let secret_num = rand::thread_rng().gen_range(1..100);

    println!("The secret number is: {secret_num}");    //秘密の数字は次の通り: {secret_num}

    println!("Input your guess?");
    let mut gu = String::new();
    io::stdin() 
        .read_line(&mut gu)
        .expect("Failed to read line");
    println!("Hello, your guess:{gu}");

}
