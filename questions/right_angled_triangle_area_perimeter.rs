// Write a code in rust to print the perimeter and area and the hypotenuse of a right angled triangle.

fn main() {
    let base : i32 = 3;
    let perpendicular : i32 = 4;
    let hypotenuse : i32 = ((base.pow(2) + perpendicular.pow(2)) as f64).sqrt() as i32;
    let area : f32 = 0.5 * base as f32 * perpendicular as f32;
    let perimeter : i32 = base + perpendicular + hypotenuse;

    println!("Base of the triangle is : {}.\nPerpendicular of the triangle is : {}.\nHypotenuse of the triangle is : {}.\nArea of the triangle is : {}.\nPerimeter of the triangle is : {}.", base, perpendicular, hypotenuse, area, perimeter);
}