use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..10);

    println!("The secret number is: {}", secret_number);   // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("aaa");

    println!("You guessed: {}", guess);     // 次のように予想しました: {}
}
