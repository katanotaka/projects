
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("na123"),
        email: String::from("ma@e.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail");
    println!("{}",user1.email);

}


//仮引数名と構造体のフィールド名が全く一緒なので、フィールド初期化省略記法

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   //rya ku ki hou 
        email,
        sign_in_count: 1,
    }
}

