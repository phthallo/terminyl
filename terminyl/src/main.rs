use std::env;
use std::io::Write;
use std::time::Duration;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let study: usize = (&args[1]).parse().unwrap();
    let rest: usize = (&args[2]).parse().unwrap();
    let session_number: usize = (&args[3]).parse().unwrap();
    for i in 0..session_number {
        println!("\nSession {}: Starting timer for {} minutes of study.\n", i+1, study);
        countdown(study);
        println!("\nStarting break for {} minutes of rest! Great work, you got this! \n", rest);
        countdown(rest);
    }
}

fn countdown(time: usize) {
    let minutes = Duration::new(60, 0);
    for i in 0..time+1 {
        let remaining_time : usize = time-i;
        let thing : usize = ((((remaining_time as f64/time as f64)*100.0)/10.0)) as usize;
        print!("\r     > {} minutes remaining!      {:>10}", remaining_time, "█".repeat(thing));
        let _ = std::io::stdout().flush();
        thread::sleep(minutes);
    }
    print!("\n");

}
