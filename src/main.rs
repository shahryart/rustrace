extern crate nix;

fn traceChild(pid: nix::unistd::Pid) {
    // Wait on the child process to stop.
    nix::sys::wait::waitpid(pid, None)
    .expect("Failed to wait on child process");

    loop {
        nix::sys::ptrace::syscall(pid, None);
    }
}

fn executeProcess() {
   nix::sys::ptrace::traceme().expect("child: traceme failed");
   let command = std::ffi::CString::new("/bin/ls").expect("CString failed");
   nix::unistd::execve(&command, &[], &[]).expect("child:execve failed");
}


fn main() {
    // Spawn a process.
    match nix::unistd::fork() {
        Ok(nix::unistd::ForkResult::Parent{child}) => traceChild(child),
        Ok(nix::unistd::ForkResult::Child) => executeProcess(),
        Err(_) => println!("fork failed")
    }

    println!("Stopped the child process");
}
