fn main() {
    let mut number: i32 = 123;
    let mut sum: i32 = 0;

    while number != 0 {
        let digit = number % 10;
        sum += digit;
        number = number / 10;
    }

    println!("The sum of the digits is : {}", sum);
}