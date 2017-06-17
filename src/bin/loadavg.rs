
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
            .and_then(|c| c.split_whitespace().next().map(|l| l.to_string()))
            .map(|l| format!("<span foreground='#B4B7B4'>\u{f109}</span> {}", l))
    });
}
