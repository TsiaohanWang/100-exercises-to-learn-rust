use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let send = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            println!("Send: {val}");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs_f64(0.5));
        }
    });
send.join().unwrap();
    loop {
        let receive = rx.recv().unwrap();
    println!("Receive: {receive}");
    }
    
}