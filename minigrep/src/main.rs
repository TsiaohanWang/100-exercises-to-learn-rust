use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }

        let handle1 = thread::spawn(|| {
            for i in 1..15 {
            println!("hi number {i} from the spawned thread from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        };

        handle1.join().unwrap();
        });
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}