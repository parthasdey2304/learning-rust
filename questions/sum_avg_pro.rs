// Write a code in Rust to print the sum, average and product of 3 numbers
fn main() {
    let a : i32 = 10;
    let b : i32 = 15;
    let c : i32 = 20;
    let sum : i32 = a + b + c;
    let avg : i32 = sum / 3;
    let pro : i32 = a * b * c;

    println!("Sum of {}, {}, {} is {}.\nAverage of {}, {}, {} is {}.\nProduct of {}, {}, {} is {}.", a, b, c, sum, a, b, c, avg, a, b, c, pro);
}