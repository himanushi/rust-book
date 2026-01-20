fn main() {
    let s1 = String::from("hello");
    let len = calc_len(&s1);

    println!("{s1} は {len} 個");
}

fn calc_len(s: &String) -> usize {
    s.len()
}
