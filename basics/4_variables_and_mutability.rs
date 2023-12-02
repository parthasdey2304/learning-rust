// this code teaches us the concept of variable and mutability in rust
fn main() {
    let a = 5;
    let mut b = 10;
    println!("Value of a is : {} and value of b is : {}", a, b);
    b += a;
    println!("The value of b after modification is : {}", b);
}