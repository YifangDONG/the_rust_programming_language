use std::borrow::Borrow;
use std::sync::{mpsc, Arc, Mutex}; //Multi-producer, single-consumer FIFO queue communication primitives.
use std::thread;
use std::time;
#[test]
fn calculate_sum_in_seperate_thread() {
    let task1 = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..6 {
            sum += i;
            thread::sleep(time::Duration::from_millis(10));
        }
        return sum;
    });
    let task2 = thread::spawn(|| {
        let mut sum = 0;
        for i in 6..11 {
            sum += i;
            thread::sleep(time::Duration::from_millis(1));
        }
        return sum;
    });
    let sum = task1.join().unwrap() + task2.join().unwrap();
    assert_eq!(55, sum);
}

#[test]
fn message_passing_one_channel() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let msg = "Hi";
        thread::sleep(time::Duration::from_millis(100));
        sender.send(msg).unwrap();
    });
    let msg = receiver.recv().unwrap();
    assert_eq!("Hi", msg);
}

#[test]
fn message_passing_two_sender() {
    let (sender, receiver) = mpsc::channel();
    let sender2 = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        let msgs = vec!["1.Hi", "1.rust", "1.welcome"];
        for msg in msgs {
            thread::sleep(time::Duration::from_millis(102));
            sender.send(msg).unwrap();
        }
    });

    thread::spawn(move || {
        let msgs = vec!["2.Hi", "2.rust", "2.welcome"];
        for msg in msgs {
            thread::sleep(time::Duration::from_millis(101));
            sender2.send(msg).unwrap();
        }
    });

    for msg in receiver {
        println!("{}", msg);
    }
}

#[test]
fn share_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut increamentor = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let task = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        increamentor.push(task);
    }
    for task in increamentor {
        task.join();
    }
    assert_eq!(10, *counter.lock().unwrap());
}
