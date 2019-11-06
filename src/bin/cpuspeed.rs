extern crate i3blocks;

use std::fs::File;
use std::io::Read;

fn main() {
    i3blocks::Script::new().on_refresh(|_| {
        File::open("/proc/cpuinfo")
            .and_then(|mut f| {
                let mut c = String::new();
                f.read_to_string(&mut c)?;
                Ok(c)
            })
            .ok()
            .and_then(|c| {
                for line in c.lines() {
                    if line.starts_with("cpu MHz") {
                        return line
                            .split_whitespace()
                            .nth(3)
                            .map(|z| z.to_string())
                            .and_then(|z| z.parse::<f64>().ok())
                            .map(|z| format!("{} MHz", z.round()));
                    }
                }
                None
            })
    });
}
