extern crate i3blocks;

use std::fs::File;
use std::io::Read;

fn main() {
    i3blocks::Script::new().on_refresh(|_| {
        File::open("/proc/loadavg")
            .and_then(|mut f| {
                let mut c = String::new();
                f.read_to_string(&mut c)?;
                Ok(c)
            })
            .ok()
            .and_then(|c| {
                c.split_whitespace()
                    .next()
                    .and_then(|l| l.parse::<f64>().ok())
            })
            .map(|l| {
                if l > 2. {
                    format!("<span foreground='#B85335'>\u{f109} {}</span>", l)
                } else if l > 1. {
                    format!("<span foreground='#FFB964'>\u{f109} {}</span> ", l)
                } else {
                    format!("\u{f109} {}", l)
                }
            })
    });
}
