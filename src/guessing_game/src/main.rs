extern crate rand;
use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);

    loop {
        print!("1〜100までの値を入力してください: ");
        io::stdout().flush().unwrap(); 
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("入力エラー。read_line()で失敗しました。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます！"),
            Ordering::Greater => println!("大きすぎます！"),
            Ordering::Equal => { 
                println!("ぴったり！　やったね！");
                break;
            }
        }
    }
}
