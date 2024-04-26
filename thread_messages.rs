use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (transmitter1, receiver) = mpsc::channel();
    let transmitter2 = mpsc::Sender::clone(&transmitter1);
    let transmitter3 = mpsc::Sender::clone(&transmitter1);
    let transmitter4 = mpsc::Sender::clone(&transmitter1);

    thread::spawn(move || {
        let duration = Duration::new(0,1);
        let num_vec: Vec<String> = vec!["One".into(), "Two".into(), "Three".into(), "Four".into()];
        for num in num_vec {
            thread::sleep(duration);
            transmitter1.send(num).unwrap();
        }
    });
    thread::spawn(move || {
        let num_vec: Vec<String> = vec!["Five".into(), "Six".into(), "Seven".into(), "Eight".into()];
        let duration = Duration::new(0,2);
        for num in num_vec {
            transmitter2.send(num).unwrap();
            thread::sleep(duration);
        }
    });
    thread::spawn(move || {
        let num_vec: Vec<String> = vec!["Alpha".into(), "Beta".into()];
        let duration = Duration::new(0,3);
        for num in num_vec {
            thread::sleep(duration);
            transmitter3.send(num).unwrap();
        }
    });
    thread::spawn(move || {
        let num_vec: Vec<String> = vec!["Fuck".into(), "Shit".into(), "Arse".into(), "Idiot".into()];
        let duration = Duration::new(0,4);
        for num in num_vec {
            transmitter4.send(num).unwrap();
            thread::sleep(duration);
        }
    });

    for received_val in receiver {
        println!("Received from thread: {}", received_val);
    }


}
