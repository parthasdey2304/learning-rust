fn main() {
    let days : i32 = 745;
    let years : i32 = days / 365;
    let mut days_left : i32 = days % 365;
    let months : i32 = days_left / 30;
    days_left = days_left % 30;
    println!("{} days is equal to {} years, {} months and {} days", days, years, months, days_left);
}