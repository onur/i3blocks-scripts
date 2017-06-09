
extern crate i3blocks;

fn main() {
    i3blocks::Script::new().on_left_click(|| {
        "Let click message".to_owned()
    }).on_refresh(move || {
        "Refresh message".to_owned()
    }).run();
}
