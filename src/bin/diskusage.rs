
extern crate i3blocks;

use std::process::Command;
use std::env;

fn main() {
    i3blocks::Script::new().on_refresh(|instance| {
        Command::new("df")
            .args(&["-h",
                    "-P",
                    "-l",
                    // try to get disk information of instance
                    // try $HOME environment variable if instance not available
                    // try / if none of them are available
                    instance.instance.as_ref()
                                     .unwrap_or(&env::var("HOME").unwrap_or("/".to_string()))])
            .output().ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .and_then(|o| o.lines().last()
                           .and_then(|l| l.split_whitespace().nth(4).map(|p| p.to_string())))
    });
}
