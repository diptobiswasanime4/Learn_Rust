fn main() {
    let mut v = vec![1, 3, 5, 7];

    for elem in &mut v {
        *elem *= 2;
        println!("{elem}");
    }
}
