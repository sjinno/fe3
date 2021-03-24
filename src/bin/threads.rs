use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use termion;

pub fn thread1() {
    let t = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    t.join().unwrap();
}

fn main() {
    // let (tx, rx) = channel();
    // thread::spawn(move || {
    //     tx.send(10).unwrap();
    // });
    // println!("{}", rx.recv().unwrap());

    // let (tx, rx) = channel();

    // for i in 0..10 {
    //     let tx = tx.clone();
    //     thread::spawn(move || {
    //         tx.send(i).unwrap();
    //     });
    // }

    // for _ in 0..10 {
    //     let j = rx.recv().unwrap();
    //     println!("{}", j);
    // }

    let v = vec![1, 2, 3, 4, 5];
    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut p = 0;
        while p != 5 {
            thread::sleep(Duration::from_secs(1));
            p += 1;
            tx.send((p, v[p])).unwrap();
        }
    });

    loop {
        let val = rx.recv().unwrap();
        if val.1 == 5 {
            println!("current idx: {}", val.0);
            break;
        }
    }
}
