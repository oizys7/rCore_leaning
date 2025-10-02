use std::env;

fn main() {
    if env::args().len() != 2 {
        println!("Usage: sleep <millis>");
        return;
    }
    let seconds = env::args().nth(1).unwrap().parse::<u64>().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(seconds));
}

fn in_rust_sbi() {
    if env::args().len() != 2 {
        println!("Usage: sleep <millis>");
        return;
    }
    let seconds = env::args().nth(1).unwrap().parse::<u64>().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(seconds));
}