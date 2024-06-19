use std::env;
use std::io::Write;
use std::time::Duration;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let study: usize = (&args[1]).parse().unwrap();
    let rest: usize = (&args[2]).parse().unwrap();
    println!("Starting timer for {study} minutes of study.\n");
    countdown(study);
    println!("Starting break for {rest} minutes of rest! Great work, you got this! \n");
    countdown(rest);
}

fn countdown(time: usize) {
    for i in 0..time {
        let remaining_time : usize = time-i;
        print!("{remaining_time} minutes remaining {} \r", "▉".repeat(time-i));
        let minutes = Duration::new(60, 0);
        let _ = std::io::stdout().flush();
        thread::sleep(minutes);
    }
}