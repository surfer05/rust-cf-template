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
> This flag tells `cargo-generate` **not** to initialize a new Git repository inside your problem folder. This is ideal if you plan to manage all your solutions within a single, larger Git repository.

### Step 4: Solve and Run

1.  Navigate into the new directory: `cd problem-A`
2.  Paste the problem's input into `input.txt`.
3.  Write your solution logic inside the `solve()` function in `src/main.rs`.
4.  Run your code from the terminal:
    ```bash
    cargo run --release < input.txt
    ```
5.  Your program's output will appear in the `output.txt` file.

---
### Customizing the Template (Using a Fork)

If you want to modify this template for your own needs, you can **fork this repository**. After creating your fork, simply update the Git URL in your local `~/.cargo/cargo-generate.toml` file to point to your forked repository address.