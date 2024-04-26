use std::thread;

fn main() {
    let mut child_threads = Vec::new();
    for i in 1..15 {
        let builder = thread::Builder::new().name(format!("mythread{}", i));
        let handle =
        builder.spawn(|| {
            println!("Hi from thread id {:?}", thread::current().id());
        }).unwrap();
        child_threads.push(handle);
    }
    println!("Hi from th emain function");
    for i in child_threads {
        i.join().unwrap();
    }
}
