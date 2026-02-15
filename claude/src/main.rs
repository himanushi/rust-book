trait Product {
    fn name(&self) -> &str;

    fn price(&self) -> f64;

    fn tax_rate(&self) -> f64;

    fn final_price(&self) -> f64 {
        self.price() * self.tax_rate()
    }
}

struct GeneralItem {
    name: String,
    price: f64,
}

impl Product for GeneralItem {
    fn name(&self) -> &str {
        &self.name
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn tax_rate(&self) -> f64 {
        1.1
    }
}

struct FoodItem {
    name: String,
    price: f64,
}

impl Product for FoodItem {
    fn name(&self) -> &str {
        &self.name
    }

    fn price(&self) -> f64 {
        self.price
    }

    fn tax_rate(&self) -> f64 {
        1.08
    }
}

fn print_receipt<T: Product>(item: &T) {
    println!("商品: {}", item.name());
    println!("  税抜き: {}円", item.price());
    println!("  税込み: {}円", item.final_price());
}

fn main() {
    let note_pc = GeneralItem {
        name: String::from("ノートPC"),
        price: 1000.0,
    };
    let bento = FoodItem {
        name: String::from("お弁当"),
        price: 500.0,
    };

    println!("--- レシート ---");
    print_receipt(&note_pc);
    print_receipt(&bento);
}
