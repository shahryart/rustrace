extern crate nix;
use rustrace::getSyscallName;
use std::env;

fn traceChild(pid: nix::unistd::Pid) {
    // Wait on the child process to stop.
    nix::sys::wait::waitpid(pid, None)
    .expect("Failed to wait on child process");

    loop {
         match nix::sys::wait::waitpid(pid, Some(nix::sys::wait::WaitPidFlag::WNOHANG)).unwrap() {
             nix::sys::wait::WaitStatus::Exited(_, _) => break,
             _ => (),
         };

        nix::sys::ptrace::syscall(pid, None);
        nix::sys::wait::waitpid(pid, None)
        .expect("Failed to stop on the next syscall");

        // Get user registers.
        let res = nix::sys::ptrace::getregs(pid).unwrap();
        println!("{}", getSyscallName(res.orig_rax as usize));
        println!("{}, {}, {}, {}, {}, {}", res.rdi, res.rsi, res.rdx, res.r10, res.r8, res.r9);

        nix::sys::ptrace::syscall(pid, None);
        nix::sys::wait::waitpid(pid, None)
        .expect("Failed to execute syscall");

        println!("{}, {}", res.rax, res.orig_rax);

    }
}

fn executeProcess(args: Vec<String>) {
   nix::sys::ptrace::traceme().expect("child: traceme failed");
   let command = std::ffi::CString::new(args.join(" ")).expect("CString failed");
   nix::unistd::execve(&command, &[], &[]).expect("child:execve failed");
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Spawn a process.
    match nix::unistd::fork() {
        Ok(nix::unistd::ForkResult::Parent{child}) => traceChild(child),
        Ok(nix::unistd::ForkResult::Child) => executeProcess(args[1..].to_vec()),
        Err(_) => println!("fork failed")
    }

    println!("Stopped the child process");
}
