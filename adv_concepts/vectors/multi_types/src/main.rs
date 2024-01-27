enum Number {
    Integer(i32),
    Float(f64),
}

fn main() {
    let mut numeric_vector: Vec<Number> = Vec::new();

    numeric_vector.push(Number::Integer(42));
    numeric_vector.push(Number::Float(3.14));
    numeric_vector.push(Number::Integer(-10));

    for num in &numeric_vector {
        match num {
            Number::Integer(value) => {
                println!("{}", value);
            }
            Number::Float(value) => {
                println!("{}", value);
            }
        }
    }
}
