fn are_types_equal<T: 'static, U: 'static>() -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>()
}

fn main() {
    let result_same = are_types_equal::<i32, i32>();
    println!("{}", result_same); // true

    let result_diff = are_types_equal::<i32, f64>();
    println!("{}", result_diff); // false
}
