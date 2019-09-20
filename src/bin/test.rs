use std::io::prelude::*;

fn main() {
    let stdout = std::io::stdout();
    let mut writer = stdout.lock();
    writeln!(writer, "Hello");
    writeln!(writer, " World");
    // lock released here
}