use std::env;
use std::io::{stdin, stdout, Read, Write};

mod pomodoro;

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    if cfg!(target_os = "windows") {
        println!("Use Linux, lmao 🐧");
        return;
    }

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please supply exactly one argument: the number of cycles");
        return;
    }

    let cycles: u32 = args[1].parse::<u32>().expect("Error parsing input");

    let mut pomodoro_timer = pomodoro::Pomodoro::new(1500, 300, cycles);
    println!("Starting cycle 1");
    let mut work_cycle_result = pomodoro_timer.start_work_cycle();

    while work_cycle_result.is_ok() {
        pause();
        println!(
            "Starting cycle {}",
            pomodoro_timer.get_iterations_done() + 1
        );
        work_cycle_result = pomodoro_timer.start_work_cycle();

        println!("{:?}", work_cycle_result);
    }
}
