use std::thread;
use std::time::Duration;
//mpsc - multiple producer, single consumer
use std::sync::mpsc;
fn main() {
    println!("\n\nEx 1:");
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from spawned thread.", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle.join().unwrap();
    }
    println!("\n\nEx 2:");
    {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a Vector: {:?}", v);
        });
        handle.join().unwrap();
    }
    println!("\n\nEx 3:");
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        let recieved = rx.recv().unwrap();
        println!("Got: {}", recieved);
    }
    println!("\n\nEx 4:");
    {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                "more".to_owned(),
                "messages".to_owned(),
                "for".to_owned(),
                "you".to_owned(),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for r in rx {
            println!("Got: {}", r);
        }
    }
}
