fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let third = vec[2]; // This will panic if the vector has less than 3 elements
    println!("The third element is: {}", third);
}