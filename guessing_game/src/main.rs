use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the Number");    
    let secret_num = rand::thread_rng().gen_range(1..100);


loop {    
    println!("Input your guess?");
    let mut gu = String::new();
    io::stdin() 
        .read_line(&mut gu)
        .expect("Failed to read line");     //エラーメッセージ

        let gu: u32 = match gu  //変数gu 再利用して定義（文字/数値変換）
            .trim()             //空白・改行・行送り文字削除
            .parse()            //変換 Result型 Ok/Err列挙子を返す
            {
                Ok(num) => num,
                Err(_) => continue,
            };
        //  .expect("Please type a number!");   //エラーメッセージ
        
        println!("Hi, your guessing:{gu}");

        match gu.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("The secret number is: {secret_num}");    //秘密の数字は次の通り: {secret_num}

}
