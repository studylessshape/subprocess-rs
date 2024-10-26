#![windows_subsystem = "windows"]

use std::{
    env, ffi::OsString, io::Error, os::windows::process::CommandExt, path::Path, process::Stdio,
};

use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use sysinfo::System;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(Error::other("Not command target!"));
    }

    let command_path = args[1].clone();
    if command_path == "--auto_launch" && args.len() >= 3 {
        return auto_launch(&args[2]);
    }

    check_program_is_running(&command_path)?;

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

fn auto_launch(command_path: &String) -> std::io::Result<()> {
    let auto = build_auto_launch(command_path).or_else(|err| Err(std::io::Error::other(err)))?;

    let is_enable = auto.is_enabled();
    let is_err = is_enable.is_err();
    if is_enable.is_ok_and(|enable| !enable) || is_err {
        println!("Enable: {}!", command_path);
        auto.enable()
    } else {
        println!("Disable: {}!", command_path);
        auto.disable()
    }
    .or_else(|err| Err(std::io::Error::other(err)))
}

fn build_auto_launch(command_path: &String) -> auto_launch::Result<AutoLaunch> {
    let args = &[command_path];
    AutoLaunchBuilder::new()
        .set_app_name("auto_start_subprocess")
        .set_app_path(env::current_exe()?.to_str().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Path to str failed",
        ))?)
        .set_args(args)
        .build()
}
