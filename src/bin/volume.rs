
extern crate i3blocks;
extern crate regex;

use std::process::Command;

enum VolumeControl {
    Up,
    Down,
    Toggle,
}


fn volume(scontrol: Option<String>) -> Option<String> {
    Command::new("amixer")
        .args(
            &[
                "-D",
                "default",
                "get",
                &scontrol.unwrap_or("Master".to_owned()),
            ],
        )
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|o| o.lines().last().and_then(|l| {
            let re = regex::Regex::new(r"\[(\d+)%\] \[(on|off)\]").unwrap();
            re.captures(l).map(|c| {
                if c.get(2).unwrap().as_str() == "off" {
                    format!("X")
                } else {
                    format!("{}%", c.get(1).unwrap().as_str())
                }
            })
        }))
}


fn volume_control(control: VolumeControl, scontrol: Option<String>) -> Option<String> {
    let scontrol = scontrol.unwrap_or("Master".to_owned());
    let args = match control {
        VolumeControl::Up => vec!["-q", "-D", "default", "sset", &scontrol, "5%+", "unmute"],
        VolumeControl::Down => vec!["-q", "-D", "default", "sset", &scontrol, "5%-", "unmute"],
        VolumeControl::Toggle => vec!["-q", "-D", "default", "sset", &scontrol, "toggle"],
    };
    Command::new("amixer").args(&args).output().ok().and_then(|_| volume(Some(scontrol.clone())))
}


fn main() {
    i3blocks::Script::new()
        .on_refresh(|i| volume(i.instance.clone()))
        .on_right_click(|i| volume_control(VolumeControl::Toggle, i.instance.clone()))
        .on_scroll_up(|i| volume_control(VolumeControl::Up, i.instance.clone()))
        .on_scroll_down(|i| volume_control(VolumeControl::Down, i.instance.clone()))
        .on_middle_click(|i| {
            let _ = Command::new("pavucontrol").output();
            volume(i.instance.clone())
        });
}
