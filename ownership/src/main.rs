fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    let mut s2 = String::from("hello");
    let r2 = &mut s2;

    println!("{}, {}", r1, r2);

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
