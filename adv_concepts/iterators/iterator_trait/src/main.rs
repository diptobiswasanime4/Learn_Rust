fn main() {
    let v1: Vec<i32> = vec![1, 3, 5, 7];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    let v2_iter = v2.iter();

    for elem in v2_iter {
        print!("{elem} ");
    }
}
