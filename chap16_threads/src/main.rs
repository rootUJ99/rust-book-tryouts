use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc}; 

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("number from created thread {}",i );
    //         thread::sleep(Duration::from_millis(i));
    //     };
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("number from main thread {}", i);
    //     thread::sleep(Duration::from_millis(i));
    // }       

    // let v = vec![1,2,3,5];
    // let handle = thread::spawn(move || {
    //     println!("whats in the vector {:?}", v)
    // });

    // handle.join().unwrap();

    // ------ channel ------
    // let (tx, rx) = mpsc::channel();

    // let tx1 = tx.clone();
    // let handle = thread::spawn(move|| {
    //     let values = [
    //             String::from("this"),
    //             String::from("is"),
    //             String::from("from"),
    //             String::from("first"),
    //             String::from("thread"),
    //             ];
    //     for i in values {
    //         tx.send(i).unwrap();
    //         thread::sleep(Duration::from_secs(2));
    //     }
    // });

    // let handle2 = thread::spawn(move||{
    //     let values = [
    //             String::from("this-"),
    //             String::from("is-"),
    //             String::from("from-"),
    //             String::from("second"),
    //             String::from("thread-"),
    //             ];
    //     for i in values {
    //         tx1.send(i).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }

    //     for recieved in rx {
    //         println!("from the transmitter {}", recieved);
    //     } 
    // });

    // handle.join().unwrap();
    // handle2.join().unwrap();
    // ------ mutex ------
    // let m = Mutex::new(6);

    // {
    //     let mut num = m.lock().unwrap();
    //     println!("num {:?}", num);
    //     *num = 10;
    // }
    
    // println!("{:?}", m);
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..10 {
        let c_m = Arc::clone(&m);
        let handle = thread::spawn(move|| {
            let mut num = c_m.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}",*m.lock().unwrap())
}
