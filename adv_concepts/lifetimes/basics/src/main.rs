fn main() {
    let r;

    {
        let x = 5;
        r = x.clone();
    }

    println!("r: {}", r);
}
