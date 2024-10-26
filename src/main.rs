#![windows_subsystem = "windows"]

use std::{env, ffi::OsString, io::Error, os::windows::process::CommandExt, path::Path, process::Stdio};

use sysinfo::System;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(Error::other("Not command target!"));
    }

    let command_path = args[1].clone();

    if let Err(err) = check_program_is_running(&command_path) {
        return Err(err);
    }

    run_command(command_path)
}

fn check_program_is_running(command_path: &String) -> std::io::Result<()> {
    let run_commands_program_name = match Path::new(command_path).file_name() {
        Some(file_name) => OsString::from(file_name).to_ascii_lowercase(),
        None => return Err(std::io::Error::other("Dont have command target")),
    };

    // println!("Command Program: {:?}", run_commands_program_name);
    
    let sys = System::new_all();

    for (_pid, process) in sys.processes() {
        let name = OsString::from(process.name()).to_ascii_lowercase();
        // println!("[pdi: {}] {:?}", _pid.as_u32(), name);
        // Has Same Process and it is running.
        if name == run_commands_program_name {
            return Err(std::io::Error::other("The same process is running!"));
        }
    }

    Ok(())
}

fn run_command(command_path: String) -> std::io::Result<()> {
    let mut child = std::process::Command::new(command_path)
            .creation_flags(0x08000000)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

    child.wait().and(Ok(()))
}