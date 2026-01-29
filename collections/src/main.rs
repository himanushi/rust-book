use std::ops::RangeFull;

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let all = &v[RangeFull]; // ✅ 書ける！
    println!("{:?}", all); // [1, 2, 3, 4, 5]
}
