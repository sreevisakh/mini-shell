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

3. **Edit `Cargo.toml`** to include dependencies:

   ```toml
   [dependencies]
   nix = "0.27"  # Provides low-level OS functions like fork, execvp, dup2
   ```

---

## 📁 Suggested File Structure

```
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

### 📝 In `src/main.rs`:

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

### ✅ Test:

* Run the shell with `cargo run`
* You should see a prompt
* Typing `exit` should close the shell
* Any other input should be echoed

---

## ⚙️ Step 3: Executing External Commands

You’ll use `nix` to fork and run commands.

### ✅ Tasks:

* Parse input into arguments
* Use `nix::unistd::fork()`
* Use `nix::unistd::execvp()`
* Use `waitpid()` to wait for the child

➡️ We will walk through each of these in the next section.

---

## 🔄 Step 4: Add Built-in Commands (cd, exit)

You’ll implement handling for `cd` and `exit` directly in the main loop.

### ✅ Tasks:

* Detect `cd <path>` and call `nix::unistd::chdir()`
* Handle errors for invalid paths
* Use `exit(0)` for `exit`

---

## 📤 Step 5: I/O Redirection

Support commands like:

```bash
echo hello > file.txt
cat < file.txt
```

### ✅ Tasks:

* Detect `<` or `>` in the input
* Use `open()` and `dup2()` to redirect stdin/stdout

---

## 🔗 Step 6: Piping with `|`

Support chaining commands like:

```bash
ls | grep txt | wc -l
```

### ✅ Tasks:

* Parse commands split by `|`
* Use `pipe()`, `fork()`, `dup2()` to set up the pipeline

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

