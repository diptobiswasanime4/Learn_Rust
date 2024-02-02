use std::{
    thread,
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {        
        for i in 1..10 {
            println!("Process 1. i = {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for j in 1..5 {
        println!("Process 2. j = {}", j);
        thread::sleep(Duration::from_millis(100));
    }
    
    handle.join().unwrap();
}
