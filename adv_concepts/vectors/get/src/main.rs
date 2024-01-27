fn main() {
    let v = vec![1, 3, 5, 7];

    let third_elem: Option<&i32> = v.get(2);

    match third_elem {
        Some(third_elem) => println!("{}", third_elem),
        None => println!("Value is out of bounds."),
    }
}
