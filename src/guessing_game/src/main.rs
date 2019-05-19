extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);

    println!("1〜100までの値を入力してください。");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("入力エラー。read_line()で失敗しました。");
    println!("入力値: {}", guess);

    let guess: u32 = guess.trim().parse()
        .expect("数値を入力してください！");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("小さすぎます。残念……"),
        Ordering::Greater => println!("大きすぎます。残念……"),
        Ordering::Equal => println!("ぴったり！　やったね！"),
    }
}
