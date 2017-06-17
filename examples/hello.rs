
extern crate i3blocks;

fn main() {
    i3blocks::Script::new().on_refresh(|instance| {
        format!("Hello World: {:?}", instance)
    });
}
