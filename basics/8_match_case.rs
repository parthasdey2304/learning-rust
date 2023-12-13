// this is how you use match case in rust we don't have switch case like java or c
fn main() {
    let case = 10; // case variable
    
    match case {
        1..=5 => println!("The number is between 1 to 5."),
        6..=10 => println!("The number is between 6 to 10."),
        _ => println!("The number is not between 1 to 10."),
    }
}