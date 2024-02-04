fn main() {
    let v = vec![1, 3, 5, 7];

    let v_iter = v.iter();

    for num in v_iter {
        println!("{num}");
    }

    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}
