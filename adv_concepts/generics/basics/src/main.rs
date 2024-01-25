fn compare_values<T>(value1: T, value2: T) -> bool
where
    T: PartialEq,
{
    value1 == value2
}

fn main() {
    let result_int = compare_values(42, 42);
    println!("{}", result_int); // true

    let result_float = compare_values(3.14, 2.71);
    println!("{}", result_float); // false

    let result_str = compare_values("hello", "world");
    println!("{}", result_str); // false
}
