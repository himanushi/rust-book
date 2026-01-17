fn main() {
    let x = 3;
    {
        let x = x + 1;
        x
    };

    {}

    println!("The value of y is: {x}");
}
