trait Animal {
    fn name(&self) -> &str;
}

struct Dog;
impl Animal for Dog {
    fn name(&self) -> &str {
        "ワンワン！"
    }
}

struct Cat;
impl Animal for Cat {
    fn name(&self) -> &str {
        "ニャー"
    }
}

fn main() {
    println!("ポチの自己紹介: {}", Dog.name());
    println!("タマの自己紹介: {}", Cat.name());
}
