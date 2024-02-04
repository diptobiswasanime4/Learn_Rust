fn main() {
    let clsr = |x| x;

    let a = clsr(5);
    let b = clsr(5.5);

    println!("{a}");
    println!("{b}");
}
