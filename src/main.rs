use std::sync::{Arc, Mutex};
use std::thread;

struct MyStruct {
    counter: i32,
}

impl MyStruct {
    fn increment(&mut self) {
        self.counter += 1;
    }
}

fn main() {
    // Wrap the struct in Arc<Mutex>
    let my_struct = Arc::new(Mutex::new(MyStruct { counter: 0 }));
    
    // Clone the Arc for each thread
    let my_struct_clone = Arc::clone(&my_struct);
    
    let handle = thread::spawn(move || {
        // Lock the mutex to get mutable access
        let mut locked = my_struct_clone.lock().unwrap();
        locked.increment();
        println!("Counter in thread: {}", locked.counter);
    });
    
    handle.join().unwrap();
    
    // Access in main thread
    let locked = my_struct.lock().unwrap();
    println!("Final counter: {}", locked.counter);
}