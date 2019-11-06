extern crate i3blocks;

use std::env;
use std::process::Command;

fn main() {
    i3blocks::Script::new().on_refresh(|_| {
        Command::new("acpi")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|o| o.split_whitespace().nth(3).map(|s| s.to_string()))
    });
}
