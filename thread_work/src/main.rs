use std::thread;
use std::time::Duration;

fn main() {
    join_thread();
    simple_thread();
    move_closure();
}
fn move_closure(){
    let v = vec![1,2,3];
    let handle = thread::spawn( move || {
        println!("Here is vector, victor {:?}", v);
    });
    handle.join().unwrap();
}
fn join_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("JOIN: Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // Putting the join here makes it look serial
    // handle.join().unwrap();
    for i in 1..5 {
        println!("JOIN: Hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // Putting the join here interleaves the messages and
    // waits for the spawned thread to finish.
    handle.join().unwrap();
}
fn simple_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("SIMPLE: Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("SIMPLE: Hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
