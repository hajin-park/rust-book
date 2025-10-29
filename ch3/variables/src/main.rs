fn main() {
    let condition: bool = true;
    let number = if condition { "bad" } else { -1 };

    println!("The value of the number is: {number}");
}
