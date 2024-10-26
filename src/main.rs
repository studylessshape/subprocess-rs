#![windows_subsystem = "windows"]

use std::{env, io::Error, os::windows::process::CommandExt, process::Child, thread};

fn main() -> std::io::Result<()> {
    let thread = thread::spawn(|| {
        let args: Vec<String> = env::args().collect();
        if args.len() <= 1 {
            return Err(Error::other("Not command target!"));
        }

        let child = std::process::Command::new(args[1].clone())
            .creation_flags(0x08000000)
            .spawn()
            .unwrap();

        let mut child_process = ChildProcess { child };

        child_process.child.wait().and(Ok(()))
    });

    thread
        .join()
        .expect("Couldn't join on the associated thread")
}

struct ChildProcess {
    child: Child,
}

impl Drop for ChildProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
