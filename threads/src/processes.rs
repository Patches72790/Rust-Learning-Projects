use nix::{
    sys::wait::waitpid,
    unistd::{fork, ForkResult},
};
use std::{
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

pub fn fork_the_process() {
    match fork().expect("Failed to fork process") {
        ForkResult::Parent { child } => {
            println!("Try to kill me! I'm the parent process!");

            waitpid(Some(child), None).unwrap();

            sleep(Duration::from_secs(20));
        }
        ForkResult::Child => {
            Command::new("/bin/echo").spawn().expect("Failed to spawn child process!");
            exit(0);
        },
    }
}
