fn factorial(number : i32) -> i32 { // this is how you write the functions with return type and do recursion
    if number == 0 {
        return 1;
    }
    else {
        return number * factorial(number - 1);
    }
}

fn main() {
    let number = 5;
    println!("The factorial of the number {} is : {}", number, factorial(number)); // this is how you invoke functions in rust
}

// 
//  * fn function_name(mut parameter_variable : variable_type) -> return_type {
//  *   body of the function;
//  *   return statements;
//  * }
//