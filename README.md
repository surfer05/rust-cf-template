# Rust Competitive Programming Template

This template provides a quick-start environment for solving competitive programming problems in Rust.

---
### ### One-Time Setup

1.  **Install `cargo-generate`**:
    ```bash
    cargo install cargo-generate
    ```

2.  **Add this template as a favorite**: Open (or create) the file `~/.cargo/cargo-generate.toml` and add the following, replacing `YOUR_GITHUB_USERNAME`:

    ```toml
    [favorites]
    cf = { git = "[https://github.com/YOUR_GITHUB_USERNAME/rust-cf-template.git](https://github.com/YOUR_GITHUB_USERNAME/rust-cf-template.git)" }
    ```

---
### ### Usage

1.  **Create a new problem folder**: Navigate to your workspace and run:
    ```bash
    # Use --vcs none to avoid creating a nested git repo
    cargo generate cf --name problem-A --vcs none
    ```

2.  **Solve the problem**:
    - `cd problem-A`
    - Paste the input into `input.txt`.
    - Write your solution in `src/main.rs`.
    - Run `cargo run --release < input.txt`.
    - Check the result in `output.txt`.