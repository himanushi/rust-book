fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    std::io::stdin().read_line(&mut index).expect("error1");

    let index: usize = index.trim().parse().expect("error2");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
