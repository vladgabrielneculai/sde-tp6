//ex 1
use nix::{
    unistd::{fork, pipe, ForkResult, close},
};
use std::{fs::{File}, io::{stdin, Write, Read}, os::unix::prelude::FromRawFd};
//ex 2
use nix::libc;
use nix::sys::signal::{self, SigHandler, Signal};
use std::thread;
use std::time::Duration;

//ex1
// fn main() {
//     //declarare pipe parinte->copil
//     let (read,write) = pipe().unwrap();

//     match unsafe { fork() } {
//         Ok(ForkResult::Parent { child, .. }) => {
//             close(read);
//                 //citire de la tastatura
//             let mut string = String::new();
//             stdin().read_line(&mut string).unwrap();
//             println!("Se trimite un mesaj...");
//             let mut f = unsafe {File::from_raw_fd(write)};
//             f.write(string.as_bytes());
//         }
//         Ok(ForkResult::Child) => {
//             close(write);
//             let mut string = String::new();
//             let mut f = unsafe{ File::from_raw_fd(read)};
//             f.read_to_string(&mut string);
//             println!("{}",string);
//         }
//         Err(_) => println!("Fork failed"),
//     }
// }

extern "C" fn handle_signal(signal: libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    println!("received signal {}: {}", signal, signal.as_str());
}

fn main() {
    let signals = [
        Signal::SIGHUP,
        Signal::SIGINT,
        Signal::SIGQUIT,
        Signal::SIGTRAP,
        Signal::SIGFPE,
    ];
    let handler = SigHandler::Handler(handle_signal);
    for signal in signals {
        unsafe { signal::signal(signal, handler) }.unwrap();
    }
    loop {
        thread::sleep(Duration::from_millis(10));
    }
}