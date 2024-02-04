fn main() {
    let v1: Vec<i32> = vec![1, 3, 4, 5, 6, 7, 10];

    let v2: Vec<_> = v1.iter().filter(|&x| x % 2 == 0).collect();

    let v2_iter = v2.iter();

    for elem in v2_iter {
        print!("{elem} ");
    }
}
