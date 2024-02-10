extern crate nix;

use nix::sched::{unshare, CloneFlags};
use std::process::Command;

mod cgroup;

fn main() {
    // Attempt to create a new PID namespace
    match unshare(CloneFlags::CLONE_NEWPID) {
        Ok(_) => println!("New PID namespace created successfully"),
        Err(e) => eprintln!("Failed to create new PID namespace: {}", e),
    }

    // Spawn a new shell process within the new namespace
    match Command::new("sh")
        .arg("-c")
        .arg("echo hello from the isolated namespace; sleep 1; ps aux")
        .spawn()
    {
        Ok(mut child) => {
            // Wait for the child process to complete
            child.wait().unwrap();
            println!("Child process within the new PID namespace has finished execution");
        }
        Err(e) => eprintln!(
            "Failed to execute child process in new PID namespace: {}",
            e
        ),
    }
}
