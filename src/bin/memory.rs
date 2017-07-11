
extern crate i3blocks;

use std::fs::File;
use std::io::Read;

fn main() {
    i3blocks::Script::new().on_refresh(|_| {
        File::open("/proc/meminfo")
            .and_then(|mut f| {
                let mut c = String::new();
                f.read_to_string(&mut c)?;
                Ok(c)
            })
            .ok()
            .map(|c| {
                (
                    // FIXME: get rid of unwraps
                    c.lines()
                        .filter(|l| l.starts_with("MemTotal"))
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                        .and_then(|l| l.parse::<u64>().ok())
                        .unwrap(),
                    c.lines()
                        .filter(|l| l.starts_with("MemAvailable"))
                        .next()
                        .and_then(|l| l.split_whitespace().nth(1))
                        .and_then(|l| l.parse::<u64>().ok())
                        .unwrap(),
                )
            })
            .map(|m| 100 - m.1 * 100 / m.0)
            .map(|p| if p > 75 {
                format!("<span color='#B85335'>\u{f2db} {}%</span>", p)
            } else {
                format!("\u{f2db} {}%", p)
            })
    });
}
