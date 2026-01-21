fn main() {
    let mut s = String::from("hello");
    let r2 = &mut s;

    {
        let r1 = &mut s;

        println!("{}, {}", r1, r2);
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
}
