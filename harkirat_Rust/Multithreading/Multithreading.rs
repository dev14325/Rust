use std::thread;


fn main(){
  let handle = thread::spawn(||{ // closure
        for i in 0..5 {
            println!("Hi from spawn thread {}",i);
        }

    });

    handle.join().unwrap(); // await 

    for i in 0..50 {
        println!("Hi from main thread {}",i);
    }
}

// interleave print -->mix