fn main() {
    let s = String::from("test aaa");
    let len = first_word(&s);
    println!("{len}");

    let slice = &s[..];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
