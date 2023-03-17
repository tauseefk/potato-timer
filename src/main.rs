use std::{thread, time::Duration};

use clap::Parser;
use notify_rust::Notification;

/// Pomodoro Timer but Potato
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Long interval length in minutes
    #[arg(short, long, default_value_t = 25)]
    long_interval: u8,

    /// Short interval length in minutes
    #[arg(short, long, default_value_t = 5)]
    short_interval: u8,
}

const NOTIFICATION_TIMEOUT_MS: i32 = 5000;

enum Task {
    Work,
    Break,
}

fn main() {
    let args = Args::parse();

    let mut task = Task::Work;
    let mut work_session_count = 0;

    loop {
        if work_session_count > 4 {
            break;
        }

        let sleep_duration = match task {
            Task::Work => {
                work_session_count += 1;
                task = Task::Break;

                Notification::new()
                    .summary("Work Session")
                    .body("Time to get to work.")
                    .icon("clock")
                    .timeout(NOTIFICATION_TIMEOUT_MS)
                    .show()
                    .expect("Failed to notify.");

                args.long_interval * 60
            }
            Task::Break => {
                task = Task::Work;

                Notification::new()
                    .summary("Break")
                    .body("Take a walk, or stare into the distance.")
                    .icon("clock")
                    .timeout(NOTIFICATION_TIMEOUT_MS)
                    .show()
                    .expect("Failed to notify.");
                args.short_interval * 60
            }
        };
        thread::sleep(Duration::from_secs(sleep_duration as u64));
    }
}
