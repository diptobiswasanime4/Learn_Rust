use std::mem;

fn main() {
    let size_of_int = mem::size_of::<i32>();
    let size_of_isize = mem::size_of::<isize>();
    let size_of_usize = mem::size_of::<usize>();

    println!("Size of i32: {} bytes", size_of_int);
    println!("Size of isize: {} bytes", size_of_isize);
    println!("Size of usize: {} bytes", size_of_usize);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let size_of_array = mem::size_of_val(&array);
    println!("Size of array: {} bytes", size_of_array);
}
