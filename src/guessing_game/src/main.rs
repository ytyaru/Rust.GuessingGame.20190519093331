extern crate rand;
use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);

    println!("1〜100までの値を入力してください。");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("入力エラー。read_line()で失敗しました。");
    println!("入力値: {}", guess);
}
