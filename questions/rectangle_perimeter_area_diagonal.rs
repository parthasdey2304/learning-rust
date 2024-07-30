// Write a code in Rust to take the length and breadth of a rectangle as input and print the perimeter, area and the length of the diagonal(using Pythagoras' Theorem)

fn main() {
    let length: i32 = 10;
    let breadth: i32 = 5;
    let perimeter : i32 = 2 * (length + breadth);
    let area : i32 = length * breadth;
    let diagonal = ((length.pow(2) + breadth.pow(2)) as f64).sqrt() as i32;

    println!(
        "The length of the rectangle is : {}cm.\nThe breadth of the rectangle is : {}cm.\nThe perimeter of the rectangle is : {}cm.\nThe area of the rectangle is : {} sq cm.\nThe length of the diagonal is : {}cm.",
        length, breadth, perimeter, area, diagonal
    );
}