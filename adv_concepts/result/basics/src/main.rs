/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/


fn safe_divide(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    if denominator == 0 {
        Err("Division by zero is not allowed.")
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    match safe_divide(10, 2) {
        Ok(result) => println!("Result = {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }

    match safe_divide(5, 0) {
        Ok(result) => println!("Result = {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
