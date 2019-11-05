extern crate i3blocks;

use std::fs::File;
use std::io::Read;

fn main() {
    // This is less sane than 4cc48147891549b96dd01255bd84308dfc2d6cbe
    // but it looks much cooler. It assumes /proc/meminfo always contains MemTotal and MemAvailable
    // and MemTotal always comes before MemAvailable
    i3blocks::Script::new().on_refresh(|_| {
        File::open("/proc/meminfo")
            .and_then(|mut f| {
                let mut c = String::new();
                f.read_to_string(&mut c)?;
                Ok(c)
            })
            .ok()
            .map(|c| {
                c.lines()
                    .filter(|l| l.starts_with("MemTotal") || l.starts_with("MemAvailable"))
                    .map(|l| {
                        l.split_whitespace()
                            .nth(1)
                            .and_then(|l| l.parse::<u64>().ok())
                            .unwrap_or(1)
                    })
                    .collect::<Vec<u64>>()
            })
            .map(|m| 100 - m[1] * 100 / m[0])
            .map(|p| {
                if p > 75 {
                    format!("<span color='#B85335'>\u{f2db} {}%</span>", p)
                } else {
                    format!("\u{f2db} {}%", p)
                }
            })
    });
}
