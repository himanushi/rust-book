use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    let secret_number = rand::thread_rng().gen_range(1..10);

    println!("The secret number is: {}", secret_number);   // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("aaa");

    let guess: u32 = guess.trim().parse().expect("error");

    println!("You guessed: {}", guess);     // 次のように予想しました: {}

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
