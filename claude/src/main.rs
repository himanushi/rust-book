trait Animal {
    fn name(&self) -> &str;
    fn speak(&self) -> &str;

    // デフォルト実装（Dogだけ上書きする）
    fn is_loud(&self) -> bool {
        false
    }
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    fn speak(&self) -> &str {
        "ワンワン！"
    }
    fn is_loud(&self) -> bool {
        true // Dogだけ上書き
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    fn speak(&self) -> &str {
        "ニャー"
    }
    // is_loud はデフォルトの false がそのまま使われる
}

// トレイト境界を使ったジェネリック関数
fn introduce<T: Animal>(animal: &T) {
    println!("{}の自己紹介: {}", animal.name(), animal.speak());
}

fn main() {
    let dog = Dog {
        name: String::from("ポチ"),
    };
    let cat = Cat {
        name: String::from("タマ"),
    };

    introduce(&dog);
    introduce(&cat);

    println!("{}はうるさい？ {}", dog.name(), dog.is_loud());
    println!("{}はうるさい？ {}", cat.name(), cat.is_loud());
}
