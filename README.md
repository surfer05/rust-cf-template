# Rust Competitive Programming Template

A ready-to-use template for solving competitive programming problems in Rust. This setup includes fast I/O and a structure for handling multiple test cases, writing output directly to a file.

## How to Use This Template

Follow these steps to set up your environment and start solving problems.

### Step 1: Prerequisites

1.  Ensure you have the Rust toolchain (including `rustc` and `cargo`) installed.
2.  Install `cargo-generate` for creating projects from templates:
    ```bash
    cargo install cargo-generate
    ```

### Step 2: Create a Local Alias (Recommended)

To avoid typing the full repository URL every time, you can create a short alias (`cf`) that points directly to this template.

1.  Open (or create) the `cargo-generate` config file located at `~/.cargo/cargo-generate.toml`.
2.  Add the following lines to the file:
    ```toml
    # In ~/.cargo/cargo-generate.toml
    [favorites]
    cf = { git = "[https://github.com/surfer05/rust-cf-template.git](https://github.com/surfer05/rust-cf-template.git)" }
    ```

### Step 3: Generate a New Problem Folder

Navigate to your workspace and run the following command to create a folder for your new problem:
   
```bash
 cargo generate cf --name problem-A --vcs none
```

> **Why use `--vcs none`?** 
> This flag tells `cargo-generate` **not** to initialize a new Git repository inside your problem folder. 

### Step 4: Solve and Run

1.  Navigate into the new directory: `cd problem-A`
2.  Paste the problem's input into `input.txt`.
3.  Write your solution logic inside the `solve()` function in `src/main.rs`.
4.  **Run your code locally** with this command:
    ```bash
    cargo run --release --features local-run < input.txt
    ```
5.  Check your result in the `output.txt` file. When you submit to an online judge, simply copy the code from `src/main.rs`; it will automatically work correctly.
