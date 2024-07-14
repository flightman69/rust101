use std::thread;

fn main() {
    let numbers: Vec<_> = (0..=1000).collect();

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("The average is {:?}", average);
}
