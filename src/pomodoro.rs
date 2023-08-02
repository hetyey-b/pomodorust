use std::process::Command;
use std::{thread, time};

const PROGRESS_BAR_SIZE: u16 = 50;

pub struct Pomodoro {
    work_length: u32,
    rest_length: u32,
    iterations: u32,
    working: bool,
    time_passed: u32,
    iterations_done: u32,
    have_to_resume: bool,
}

impl Pomodoro {
    pub fn new(work_length_sec: u32, rest_length_sec: u32, iterations: u32) -> Self {
        return Self {
            work_length: work_length_sec,
            rest_length: rest_length_sec,
            iterations,
            working: false,
            time_passed: 0,
            iterations_done: 0,
            have_to_resume: false,
        };
    }

    pub fn start_work_cycle(&mut self) -> Result<String, String> {
        fn progress_bar_status(current: u32, total: u32) -> u16 {
            let cur = current as f64;
            let tot = total as f64;
            let percentage = (cur / tot) * (PROGRESS_BAR_SIZE as f64);
            return percentage.round() as u16;
        }

        if self.working {
            return Err("Already working!".to_owned());
        }
        if self.iterations_done >= self.iterations {
            return Err("All iterations done!".to_owned());
        }
        self.time_passed = 0;
        self.iterations_done += 1;
        self.working = true;

        if self.have_to_resume {
            Command::new("sh")
                .arg("-c")
                .arg("playerctl play")
                .output()
                .expect("failed to execute 'playerctl pause'");
        }

        let one_second = time::Duration::from_secs(1);
        for i in 0..self.work_length {
            thread::sleep(one_second);

            print!("{}c", 27 as char);

            println!("Cycle {} - Work", self.iterations_done + 1);

            let n = progress_bar_status(i + 1, self.work_length);
            for _i in 0..n {
                print!("|");
            }
            for _i in n..PROGRESS_BAR_SIZE {
                print!("-");
            }
            print!("\n");
        }

        self.working = false;

        let output = Command::new("sh")
            .arg("-c")
            .arg("playerctl status")
            .output()
            .expect("failed to execute 'playerctl status'");

        let playerctl_status = String::from_utf8_lossy(&output.stdout);
        self.have_to_resume = playerctl_status == "Playing\n";

        Command::new("sh")
            .arg("-c")
            .arg("playerctl pause")
            .output()
            .expect("failed to execute 'playerctl pause'");

        Command::new("sh")
            .arg("-c")
            .arg("aplay ~/Music/pomodoro_notification.wav")
            .output()
            .expect("failed to execute 'playerctl pause'");

        for i in 0..self.rest_length {
            thread::sleep(one_second);

            print!("{}c", 27 as char);

            println!("Cycle {} - Rest", self.iterations_done + 1);

            let n = progress_bar_status(i + 1, self.rest_length);
            for _i in 0..n {
                print!("|");
            }
            for _i in n..PROGRESS_BAR_SIZE {
                print!("-");
            }
            print!("\n");
        }

        return Ok("Ok".to_owned());
    }
}
