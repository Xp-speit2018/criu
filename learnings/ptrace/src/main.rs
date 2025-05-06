use std::{collections::HashMap, i32::MAX, os::unix::process::CommandExt, process::Command};

use nix::{
    libc, sys::{
        ptrace,
        signal, 
        wait::{
            waitpid,
            WaitStatus
        }
    }, unistd::Pid
};
use owo_colors::OwoColorize;

use procfs::process::Process;

fn print_proc_status(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    let proc = Process::new(pid as i32)?;
    println!("Process ID: {}, State: {:?}", proc.pid, proc.status().unwrap().state);
    Ok(())
}


fn naive_ptrace() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new command to run the test program
    let mut command = Command::new("./test");
    
    // Enable tracing for the child process
    unsafe {
        // traceme() is a syscall that allows the calling process to be traced by its parent
        command.pre_exec(|| -> std::io::Result<()> {
            ptrace::traceme().map_err(|e| std::io::Error::from(e))?;
            Ok(())
        });
    }

    // Spawn the child process
    let child = command.spawn()?;
    
    // Get the PID of the child process
    let child_pid_raw = child.id();
    let child_pid = Pid::from_raw(child_pid_raw as i32); // we need i32 here, not u32

    println!("Parent's PID: {}", std::process::id());
    // Wait for initial stop from PTRACE_TRACEME
    match waitpid(child_pid, None)? {
        WaitStatus::Stopped(pid, signal::SIGTRAP) => {
            println!("Child {} stopped by PTRACE_TRACEME", pid);
        }
        _ => return Err("Unexpected initial child state".into()),
    }

    const MAX_SYSCALLS: i32 = 200;
    // capture syscalls for MAX_SYSCALLS times then kill the child process
    for i in 0..MAX_SYSCALLS {
        ptrace::syscall(child_pid, None)?; // Arranges for the tracee to be stopped at the next entry
        _ = waitpid(child_pid, None)?; // block the tracer until the tracee stops

        // collecting tracee registers
        // linux naming of registers can be found at https://github.com/torvalds/linux/blob/master/arch/x86/include/asm/user_64.h
        // crate-specific naming of registers can be found at https://docs.rs/libc/0.2.155/libc/struct.user_regs_struct.html
        // register conventions for linux x86_64 syscall can be found at https://www.uclibc.org/docs/psABI-x86_64.pdf, A.2.1
        let regs = ptrace::getregs(child_pid)?;
        println!(
            "[{}/{}]  {}({:x}, {:x}, {:x}, ...) = {:x}",
            i+1,
            MAX_SYSCALLS,
            regs.orig_rax, // syscall number, https://github.com/torvalds/linux/blob/v6.7/arch/x86/entry/syscalls/syscall_64.tbl
            regs.rdi, // 1st arg
            regs.rsi, // 2nd arg
            regs.rdx, // 3rd arg
            regs.rax, // return value
        );
        
        ptrace::syscall(child_pid, None)?; // Arranges for the tracee to be stopped at the next exit
        _ = waitpid(child_pid, None)?; // block the tracer until the tracee stops
    }
    println!("Killing child process {}", child_pid);
    signal::kill(child_pid, signal::SIGKILL)?;

    Ok(())
}

fn does_child_stop_on_creation() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new command to run the test program
    let mut command = Command::new("./test");
    
    // Enable tracing for the child process
    unsafe {
        command.pre_exec(|| {
            ptrace::traceme().map_err(|e| e.into())
        });
    }

    // Spawn the child process
    let child = command.spawn()?;
    
    // Get the PID of the child process
    let child_pid_raw = child.id();
    let child_pid = Pid::from_raw(child_pid_raw as i32); // we need i32 here, not u32
    
    println!("Parent's PID: {}", std::process::id());
    println!("Child process PID: {}", child_pid_raw);

    // Wait for the child process to stop(by SIGTRAP installed by PTRACE_TRACEME)
    let status = waitpid(child_pid, None)?;
    
    println!("Child process stopped with status: {:?}", status);

    ptrace::syscall(child_pid, None)?;
    println!("Child process resumed after stopping");
    
    println!("Main process sleeping for 2 seconds");
    unsafe {
        libc::sleep(2);
    }
    // Check the status of the child process, do not use waitpid



    println!("Main process exiting");
    Ok(())
}

fn does_child_gets_killed_when_trapped_orphaned() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new command to run the test program
    let mut command = Command::new("./test");

    // Spawn the child process
    let child = command.spawn()?;
    // Get the PID of the child process
    let child_pid_raw = child.id();
    let child_pid = Pid::from_raw(child_pid_raw as i32); // we need i32 here, not u32
    println!("Parent's PID: {}", std::process::id());
    println!("Child process PID: {}", child_pid_raw);

    // Check the status of the child process
    print_proc_status(child_pid_raw)?;

    unsafe{
        libc::sleep(2);
    }
    // send a sigtrap signal to the child process
    signal::kill(child_pid, signal::SIGTRAP)?;
    print_proc_status(child_pid_raw)?;
    unsafe{
        libc::sleep(2);
    }
    print_proc_status(child_pid_raw)?;
    Ok(())
    
}

// fn beautified_ptrace() -> Result<(), Box<dyn std::error::Error>> {
//     let json: serde_json::Value = serde_json::from_str(include_str!("syscall.json"))?;
//     let syscall_table: HashMap<u64, String> = json["aaData"]
//     .as_array()
//     .unwrap()
//     .iter()
//     .map(|item| {
//         (
//             item[0].as_u64().unwrap(),
//             item[1].as_str().unwrap().to_owned(),
//         )
//     })
//     .collect();

//     // Create a new command to run the test program
//     let mut command = Command::new("./test");
    
//     // Enable tracing for the child process
//     unsafe {
//         command.pre_exec(|| {
//             ptrace::traceme().map_err(|e| e.into())
//         });
//     }

//     // Spawn the child process
//     let child = command.spawn()?;
//     // Get the PID of the child process
//     let child_pid_raw = child.id();
//     let child_pid = Pid::from_raw(child_pid_raw as i32); 
//     println!("Child process PID: {}", child_pid_raw);
    
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run the naive ptrace function
    naive_ptrace()?;

    // Run the does_child_stop_on_creation function
    // does_child_stop_on_creation()?;

    // does_child_gets_killed_when_trapped_orphaned()?;

    Ok(())
}