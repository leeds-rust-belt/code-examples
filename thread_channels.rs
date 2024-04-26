use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main(){
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("tx1-1"),
            String::from("tx1-2"),
            String::from("tx1-3"),
            String::from("tx1-4"),
            String::from("tx1-5"),
            String::from("tx1-6"),
            String::from("tx1-7"),
            String::from("tx1-8"),
            String::from("tx1-9"),
            String::from("tx1-a"),
            String::from("tx1-b"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("tx-1"),
            String::from("tx-2"),
            String::from("tx-3"),
            String::from("tx-4"),
            String::from("tx-5"),
            String::from("tx-6"),
            String::from("tx-7"),
            String::from("tx-8"),
            String::from("tx-9"),
            String::from("tx-a"),
            String::from("tx-b"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(19));
        }
    });


    for received in rx {
        println!("Got: {}", received);
    }

}
