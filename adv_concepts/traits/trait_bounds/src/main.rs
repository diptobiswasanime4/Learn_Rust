trait Printable {
    fn print(&self);
}

struct TestType;

impl Printable for TestType {
    fn print(&self) {
        println!("Printing Test Type.")
    }
}

fn main() {
    let test_type = TestType;
    print_val(test_type);
}

fn print_val<T: Printable>(val: T) {
    val.print();
}
