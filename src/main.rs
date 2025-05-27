mod executor;

use std::io::{self, Write};
use std::process::exit;
use nix::sys::wait::waitpid;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::dup2;
use executor::run_execvp;

fn main() {
    loop {
        let dir = nix::unistd::getcwd().unwrap();
        print!("mini-shell({})> ", dir.display());

        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        let command: &str = input.trim();
        if command == "exit" {
            exit(0);
        }

        let tokens: Vec<&str>= command.split_ascii_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if tokens[0] == "cd" {
            let target = tokens.get(1).unwrap_or(&"/").to_string();
            if let Err(err) = nix::unistd::chdir(target.as_str()) {
                eprintln!("cd :{}", err);
            }
            continue;
        }

        match unsafe { nix::unistd::fork() } {
            Ok(nix::unistd::ForkResult::Child) => {
                if let Some((cmd, filename)) = command.split_once('>') {
                    let fd = open(filename.trim(), OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC, Mode::S_IRUSR | Mode::S_IWUSR).unwrap();
                    let _ = dup2(fd, 1);
                    run_execvp(cmd);
                } else if let Some((cmd, filename)) = command.split_once('<'){
                  let fd = open(filename.trim(), OFlag::O_RDONLY, Mode::empty()).unwrap();
                    let _ = dup2(fd, 0);
                    run_execvp(cmd);
                } else {
                    run_execvp(command);
                }   
            }
            Ok(nix::unistd::ForkResult::Parent { child }) => {
               let _  = waitpid(child, None);
            }
            Err(err) => {
                eprintln!("Fork failed: {}", err);
            }
        }
    }
}
