# 🧪 Systems Programming Tutorial: Build a Unix-like Shell in Rust

## 🎯 Objective

Welcome! In this guided tutorial, you'll build **Mini-Shell**, a basic Unix-like command-line shell in **Rust**. This shell will support command execution, piping, I/O redirection, and background processes. No prior experience with Rust or systems programming is required — we'll guide you every step of the way.

---

## 📦 Step 1: Setup Your Rust Environment

1. **Install Rust:** If you haven't already, install Rust using [rustup](https://rustup.rs/):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Create the project:**

   ```bash
   cargo new mini-shell
   cd mini-shell
   ```

3. **Edit `Cargo.toml` to include dependencies**:

   ```toml
   [dependencies]
   nix = "0.27"  # Provides low-level OS functions like fork, execvp, dup2
   ```

---

## 📁 Suggested File Structure

```text
mini-shell/
├── src/
│   ├── main.rs        # Shell entry point
│   ├── parser.rs      # Input parsing
│   ├── executor.rs    # Command execution
│   └── utils.rs       # Helper functions
├── tests/             # Shell behavior tests
├── Cargo.toml         # Package configuration
└── README.md
```

---

## 🧩 Step 2: Build the Prompt Loop

Let’s now build your shell loop. Below is a simple Rust program that shows a prompt, reads user input, and exits when the user types `exit`.

### 🧱 Full Code

```rust
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("mini-shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line");
            continue;
        }

        let command = input.trim();
        if command == "exit" {
            println!("Goodbye!");
            exit(0);
        }

        println!("You entered: {}", command);
    }
}
```

### 🔍 Explanation

* **`print!` + `flush()`**: Prints the prompt without a newline and forces it to appear immediately.
* **`read_line()`**: Reads the user's input from the terminal.
* **`trim()`**: Cleans up newline and space characters.
* **`exit(0)`**: Terminates the shell cleanly if the user types `exit`.
* **Echo**: Everything else is simply printed back as a placeholder for future command execution.

---

## ⚙️ Step 3: Executing External Commands

### 🧰 Create a Function to Run execvp

Let’s encapsulate the execution logic into a reusable function you can call after parsing commands or redirecting input/output.

### 📦 Function Definition

Add the following to a new file like `executor.rs`, or directly in `main.rs` for now:

```rust
use nix::unistd::execvp;
use std::ffi::CString;

pub fn run_execvp(tokens: Vec<&str>) -> ! {
    let cstr_args: Vec<CString> = tokens.iter()
        .map(|&s| CString::new(s).unwrap())
        .collect();

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
```

### 🧪 Usage

In your `fork()` block, just call:

```rust
run_execvp(tokens);
```

> ⚠️ Make sure to import the function at the top of `main.rs` if it's in `executor.rs`:
>
> ```rust
> mod executor;
> use executor::run_execvp;
> ```
>
> If you're getting `not found in this scope`, this import is likely missing.

This makes your command execution clean and reusable for piping, redirection, and background process logic.

You’ll now make your shell capable of running actual external programs like `ls`, `pwd`, or `echo`.

### 🔍 What you’ll learn:

* How to split a command string into arguments
* How to create a child process using `fork()`
* How to replace the child process with the desired command using `execvp()`
* How to wait for the child to finish using `waitpid()`

### 🪛 Step-by-step:

#### 1. **Split the input command into arguments**

```rust
let tokens: Vec<&str> = command.split_whitespace().collect();
```

#### 2. **Ignore empty commands**

```rust
if tokens.is_empty() {
    continue;
}
```

#### 3. **Use ****`fork()`**** to create a child process**

Add to `Cargo.toml`:

```toml
nix = { version = "0.27", features = ["process"] }
```

Then in code:

```rust
match unsafe { nix::unistd::fork() } {
    Ok(nix::unistd::ForkResult::Child) => {
        // In child
    },
    Ok(nix::unistd::ForkResult::Parent { child }) => {
        // In parent
    },
    Err(err) => {
        eprintln!("Fork failed: {}", err);
    }
}
```

#### 4. **Execute the command in the child process**

```rust
use std::ffi::CString;

let cstr_args: Vec<CString> = tokens.iter()
    .map(|&s| CString::new(s).unwrap())
    .collect();

let prog = &cstr_args[0];
let args = &cstr_args[..];

let result = nix::unistd::execvp(prog, args);
match result {
    Ok(_) => {}, // Should never reach here
    Err(err) => {
        eprintln!("Execution failed: {}", err);
        std::process::exit(1);
    }
}
```

#### 5. **Wait for the child process to finish (in parent)**

```rust
use nix::sys::wait::waitpid;

let _ = waitpid(child, None);
```

---

## 🔄 Step 4: Add Built-in Commands (cd, exit)

You’ve now added external command support. But commands like `cd` and `exit` are special — they are handled by the shell process itself because they need to affect the shell's state (like its current working directory).

### 🔍 Why `cd` and `exit` need special treatment:

* `cd` changes the current working directory of the shell itself. If you run it in a child process, the parent shell won't be affected.
* `exit` must terminate the shell process — so it can’t be delegated to a child process either.

### 👨‍💻 What you'll do:

* Intercept input before forking.
* If the command is `cd`, extract the path and call `chdir()`.
* If the command is `exit`, use `std::process::exit(0)` to quit.

### 🧱 Example code snippet:

```rust
if tokens[0] == "cd" {
    let target = tokens.get(1).unwrap_or(&"/").to_string();
    if let Err(err) = nix::unistd::chdir(target.as_str()) {
        eprintln!("cd: {}", err);
    }
    continue;
}

if tokens[0] == "exit" {
    println!("Exiting mini-shell...");
    std::process::exit(0);
}
```

This logic should appear **before** forking and executing external commands.

### ✅ Tasks:

* Detect `cd <path>` and call `nix::unistd::chdir()`
* Handle errors for invalid paths
* Use `exit(0)` for `exit`

---

## 📤 Step 5: I/O Redirection

Now that your shell can run commands, it's time to support redirecting input and output using `<` and `>` — just like in a real Unix shell.

### 🔍 What is I/O Redirection?

* `>` means: write the output of the command to a file instead of the terminal.
* `<` means: take the input of the command from a file instead of the keyboard.

### 💡 Example Use Cases:

```bash
echo hello > output.txt     # Writes 'hello' to output.txt
cat < output.txt            # Reads from output.txt instead of stdin
```

### 🧱 How to Implement:

1. **Detect redirection operators in the input.**
2. **Split the command into actual program and file name.**
3. **Open the file using `nix::fcntl::open`.**
4. **Use `nix::unistd::dup2` to replace `stdin` or `stdout`.**
5. **Then call `execvp()` as usual.**

### 🧪 Example Snippet:

```rust
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::dup2;
use std::os::unix::io::RawFd;

// Inside child process before execvp:
if let Some((cmd, filename)) = command.split_once('>') {
    let fd: RawFd = open(filename.trim(), OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC, Mode::S_IRUSR | Mode::S_IWUSR).unwrap();
    dup2(fd, 1).unwrap(); // 1 = stdout
    // then execvp(cmd)
} else if let Some((cmd, filename)) = command.split_once('<') {
    let fd: RawFd = open(filename.trim(), OFlag::O_RDONLY, Mode::empty()).unwrap();
    dup2(fd, 0).unwrap(); // 0 = stdin
    // then execvp(cmd)
}
```

### 🔍 Explanation:

* **`split_once('>') / split_once('<')`**: Separates the actual command from the filename.
* **`open()`**: Opens the file with the correct mode (write or read).
* **`dup2(fd, 1)`**: Redirects the file descriptor to stdout (1) or stdin (0).
* This only affects the child process and leaves the shell’s I/O untouched.
* You should run `execvp(cmd)` right after setting up redirection.

### ⚠️ Tip:

Only do the `open()` and `dup2()` logic inside the **child process**, right before calling `execvp()`. This prevents your shell process from affecting its own stdin/stdout.

### ✅ Tasks:

* Detect `<` or `>` in the input
* Use `open()` and `dup2()` to redirect stdin/stdout

---

## 🔗 Step 6: Piping with `|`

Pipes allow the output of one command to become the input of another — a key feature of any Unix-like shell.

### 🔍 What is Piping?

The shell connects multiple commands using the pipe (`|`) symbol. For example:

```bash
ls | grep txt | wc -l
```

This command does the following:

* `ls` lists files
* The output is sent to `grep txt` to filter files containing "txt"
* That result is passed to `wc -l` to count the lines

### 🧱 How to Implement:

1. **Split the input by `|` into separate commands**
2. **Create a pipe between each pair of commands**
3. **For each command:**

   * `fork()` a child process
   * Set up `dup2()` to redirect `stdin` and `stdout` appropriately
   * Call `execvp()`
4. **Use `waitpid()` in the parent to wait for all children**

### 🧪 Example Strategy:

You’ll repeat something like this for each stage:

* For the first command: redirect its stdout to pipe's write end
* For the middle commands: redirect stdin and stdout
* For the last command: redirect its stdin to the last pipe's read end

### 🧩 Key syscalls used:

* `pipe()` from `nix::unistd` to create pipes
* `dup2()` to redirect input/output
* `close()` to close unused pipe ends

### ✅ Tasks:

* Parse commands split by `|`
* Use `pipe()`, `fork()`, `dup2()` to set up the pipeline

### 🧪 Example: Multiple Piping (`cmd1 | cmd2 | cmd3`)

```rust
use nix::unistd::{pipe, fork, dup2, close, ForkResult};
use nix::sys::wait::waitpid;
use std::ffi::CString;

let parts: Vec<&str> = command.split('|').map(|s| s.trim()).collect();
let num_cmds = parts.len();
let mut fds = vec![];

// Create pipes for N-1 links
for _ in 0..num_cmds - 1 {
    fds.push(pipe().expect("pipe failed"));
}

for i in 0..num_cmds {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // If not first command, read from previous pipe
            if i > 0 {
                dup2(fds[i - 1].0, 0).unwrap();
            }
            // If not last command, write to next pipe
            if i < num_cmds - 1 {
                dup2(fds[i].1, 1).unwrap();
            }
            // Close all pipe fds
            for (r, w) in &fds {
                let _ = close(*r);
                let _ = close(*w);
            }
            let tokens: Vec<CString> = parts[i].split_whitespace()
                .map(|s| CString::new(s).unwrap())
                .collect();
            execvp(&tokens[0], &tokens).expect("execvp failed");
        },
        Ok(ForkResult::Parent { .. }) => continue,
        Err(err) => eprintln!("Fork failed: {}", err),
    }
}

// Parent: close all fds and wait for all children
for (r, w) in fds {
    let _ = close(r);
    let _ = close(w);
}
for _ in 0..num_cmds {
    waitpid(None, None).ok();
}
```

### 🔍 Explanation:

* **`command.split('|')`**: Breaks the input string into separate commands.
* **`pipe()`**: Creates a new read/write pipe between commands.
* **`fork()`**: Spawns a child process for each command.
* **`dup2()`**: Replaces `stdin` or `stdout` with the appropriate pipe end.
* **`close()`**: Prevents leaking file descriptors.
* **`execvp()`**: Runs each command in its process.
* **Parent waits** for all child processes to complete.rust
  use nix::unistd::{pipe, fork, dup2, close, ForkResult};
  use nix::sys::wait::waitpid;
  use std::ffi::CString;

let parts: Vec<\&str> = command.split('|').map(|s| s.trim()).collect();
let num\_cmds = parts.len();
let mut fds = vec!\[];

// Create pipes for N-1 links
for \_ in 0..num\_cmds - 1 {
fds.push(pipe().expect("pipe failed"));
}

for i in 0..num\_cmds {
match unsafe { fork() } {
Ok(ForkResult::Child) => {
// If not first command, read from previous pipe
if i > 0 {
dup2(fds\[i - 1].0, 0).unwrap();
}
// If not last command, write to next pipe
if i < num\_cmds - 1 {
dup2(fds\[i].1, 1).unwrap();
}
// Close all pipe fds
for (r, w) in \&fds {
let \_ = close(\*r);
let \_ = close(\*w);
}
let tokens: Vec<CString> = parts\[i].split\_whitespace()
.map(|s| CString::new(s).unwrap())
.collect();
execvp(\&tokens\[0], \&tokens).expect("execvp failed");
},
Ok(ForkResult::Parent { .. }) => continue,
Err(err) => eprintln!("Fork failed: {}", err),
}
}

// Parent: close all fds and wait for all children
for (r, w) in fds {
let \_ = close(r);
let \_ = close(w);
}
for \_ in 0..num\_cmds {
waitpid(None, None).ok();
}

````

### 🔍 Explanation:
- **`command.split('|')`**: Breaks the input string into separate commands.
- **`pipe()`**: Creates a new read/write pipe between commands.
- **`fork()`**: Spawns a child process for each command.
- **`dup2()`**: Replaces `stdin` or `stdout` with the appropriate pipe end.
- **`close()`**: Prevents leaking file descriptors.
- **`execvp()`**: Runs each command in its process.
- **Parent waits** for all child processes to complete.rust
use nix::unistd::{pipe, fork, dup2, close, ForkResult};
use nix::sys::wait::waitpid;
use std::ffi::CString;

let parts: Vec<&str> = command.split('|').map(|s| s.trim()).collect();
let num_cmds = parts.len();
let mut fds = vec![];

// Create pipes for N-1 links
for _ in 0..num_cmds - 1 {
    fds.push(pipe().expect("pipe failed"));
}

for i in 0..num_cmds {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // If not first command, read from previous pipe
            if i > 0 {
                dup2(fds[i - 1].0, 0).unwrap();
            }
            // If not last command, write to next pipe
            if i < num_cmds - 1 {
                dup2(fds[i].1, 1).unwrap();
            }
            // Close all pipe fds
            for (r, w) in &fds {
                let _ = close(*r);
                let _ = close(*w);
            }
            let tokens: Vec<CString> = parts[i].split_whitespace()
                .map(|s| CString::new(s).unwrap())
                .collect();
            execvp(&tokens[0], &tokens).expect("execvp failed");
        },
        Ok(ForkResult::Parent { .. }) => continue,
        Err(err) => eprintln!("Fork failed: {}", err),
    }
}
// Parent: close all fds and wait for all children

for (r, w) in fds {
    let _ = close(r);
    let _ = close(w);
}
for _ in 0..num_cmds {
    waitpid(None, None).ok();
}
```rust
use nix::unistd::{pipe, fork, dup2, close, ForkResult};
use nix::sys::wait::waitpid;
use std::ffi::CString;

let parts: Vec<&str> = command.split('|').collect();
if parts.len() == 2 {
    let (cmd1, cmd2) = (parts[0].trim(), parts[1].trim());
    let (read_end, write_end) = pipe().expect("pipe failed");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            // First command - write to pipe
            dup2(write_end, 1).unwrap(); // stdout → pipe write
            close(read_end).unwrap();
            close(write_end).unwrap();

            let tokens: Vec<CString> = cmd1.split_whitespace().map(|s| CString::new(s).unwrap()).collect();
            execvp(&tokens[0], &tokens).expect("execvp failed");
        },
        Ok(ForkResult::Parent { .. }) => {
            match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    // Second command - read from pipe
                    dup2(read_end, 0).unwrap(); // stdin ← pipe read
                    close(write_end).unwrap();
                    close(read_end).unwrap();

                    let tokens: Vec<CString> = cmd2.split_whitespace().map(|s| CString::new(s).unwrap()).collect();
                    execvp(&tokens[0], &tokens).expect("execvp failed");
                },
                Ok(ForkResult::Parent { .. }) => {
                    // Close pipe in parent
                    close(write_end).unwrap();
                    close(read_end).unwrap();

                    waitpid(None, None).ok();
                    waitpid(None, None).ok();
                },
                Err(err) => eprintln!("Fork failed: {}", err),
            }
        },
        Err(err) => eprintln!("Fork failed: {}", err),
    }
}
````

---

## 👤 Step 7: Background Processes with `&`

Run commands like:

```bash
sleep 5 &
```

### ✅ Tasks:

* Detect trailing `&`
* Skip `waitpid()`
* Print the background PID

---

## 🧼 Step 8: Signal Handling & UX Polish

Handle Ctrl+C and Ctrl+D properly.

### ✅ Tasks:

* Use `signal_hook` crate to catch signals
* Prevent Ctrl+C from killing the shell
* Exit on Ctrl+D (EOF)

---

## 🧪 Step 9: Testing and Automation

Create simple bash scripts in `tests/` to validate shell behavior.

### Example: `tests/test_echo.sh`

```bash
#!/bin/bash
cargo run <<EOF
echo hello > test.txt
cat < test.txt
exit
EOF

grep -q "hello" test.txt && echo "✅ Passed" || echo "❌ Failed"
```

Run all tests:

```bash
chmod +x tests/*.sh
for f in tests/*.sh; do echo "Running $f"; bash "$f"; done
```

---

## 📚 References

* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [Linux man pages](https://man7.org/linux/man-pages/)
* [MIT OS Course](https://pdos.csail.mit.edu/6.828/)
* [nix crate](https://docs.rs/nix/latest/nix/)

---

## 💡 Shell Name

* **Mini-Shell** (official project name)

---

You're now ready to start building Mini-Shell — your very own Unix shell in Rust! Let me know when you're ready for code walkthroughs or implementation help for any phase.

