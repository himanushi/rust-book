struct GeneralItem {
    name: String,
    price: f64,
}

impl GeneralItem {
    fn tax_rate(&self) -> f64 {
        1.1
    }

    fn final_price(&self) -> f64 {
        self.price * self.tax_rate()
    }
}

struct FoodItem {
    name: String,
    price: f64,
}

impl FoodItem {
    fn tax_rate(&self) -> f64 {
        1.08
    }

    fn final_price(&self) -> f64 {
        self.price * self.tax_rate()
    }
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
}
