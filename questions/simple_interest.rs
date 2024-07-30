// Write a code in Rust to calculate the simple interest.
fn main() {
    let principle : f32 = 300000.0;
    let rate : f32 = 9.5;
    let time : f32 = 10.0;
    let simple_interest : f32 = (principle * rate * time) / 100.0;
    let amount : f32 = simple_interest + principle;

    println!("Simple Interest on principle amount of {} at rate of {}% for {} years is {}.\nAmount after the interest is : {} ", principle, rate, time, simple_interest, amount);
}