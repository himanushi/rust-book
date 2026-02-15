trait Animal {
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        "ワンワン！"
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        "ニャー"
    }
}

fn main() {
    let dog = Dog {
        name: String::from("ワンワン"),
    };
    let cat = Cat {
        name: String::from("ニャー"),
    };

    println!(
        "ポチの自己紹介: {}",
        Dog {
            name: String::from("ワンワン")
        }
        .name()
    );
    println!("タマの自己紹介: {}", Cat.name());
}
