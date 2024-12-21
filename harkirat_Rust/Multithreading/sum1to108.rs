use std::sync::mpsc; // multiple producer single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // transmitter, receiver
    let n: u64 = 1_000_000_000; // Sum from 1 to 10^9
    let num_threads = 8; // Number of threads
    let chunk_size = n / num_threads; // Split the range into equal parts

    for i in 0..num_threads {
        let tx = tx.clone(); // Clone transmitter for each thread
        let start = i * chunk_size + 1;
        let end = if i == num_threads - 1 { n } else { (i + 1) * chunk_size };
        
        thread::spawn(move || {
            let sum: u64 = (start..=end).sum();
            tx.send(sum).unwrap(); // Send partial sum to the main thread
        });
    }

    // Collect and sum up the results from all threads
    let mut total_sum: u64 = 0;
    for _ in 0..num_threads {
        let partial_sum = rx.recv().unwrap();
        total_sum += partial_sum;
    }

    println!("The total sum from 1 to {} is: {}", n, total_sum);
}
