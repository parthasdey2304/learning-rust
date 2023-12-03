// this code teaches us the concept of variable and mutability in rust
fn main() {
    let a = 5; // a is an immutable variable that means its value cannot be changed once declared just like a constant
    let mut b = 10; // on the contrary b is a mutable variable its value can be modified later its done using the mut keyword
    println!("Value of a is : {} and value of b is : {}", a, b);
    b += a; // value of b got modified
    println!("The value of b after modification is : {}", b);
}