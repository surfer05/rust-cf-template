// main.rs

use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {
    // ---- Input Setup ----
    // Reads from standard input, which we'll pipe from input.txt
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    #[cfg(feature = "local-run")]
    let mut writer = {
        let output_file = File::create("output.txt").unwrap();
        BufWriter::new(output_file)
    };

    #[cfg(not(feature = "local-run"))]
    let mut writer = {
        let stdout = io::stdout();
        BufWriter::new(stdout.lock())
    };

    // Read the number of test cases
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    // Loop through each test case
    for _ in 0..t {
        solve(&mut reader, &mut writer);
    }
}

/// This is where you write the solution for a single test case.
fn solve(reader: &mut impl BufRead, writer: &mut impl Write) {
    // TODO: Write your solution logic for the problem here.
}
