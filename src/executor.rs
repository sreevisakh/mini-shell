use nix::unistd::execvp;
use std::ffi::CString;

pub fn run_execvp(command: &str) -> ! {
     let tokens: Vec<&str>= command.split_ascii_whitespace().collect();
    let cstr_args: Vec<CString> =
        tokens.iter().map(|&s| CString::new(s).unwrap()).collect();
    let prog = &cstr_args[0];
    let args = &cstr_args[..];

    match execvp(prog, args) {
        Ok(_) => unreachable!(),
        Err(err) => {
            eprintln!("Execution failed: {}", err);
            std::process::exit(1);
        }
    }
}