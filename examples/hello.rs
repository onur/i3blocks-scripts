extern crate i3blocks;

fn main() {
    i3blocks::Script::new().on_refresh(|instance| Some(format!("Hello World: {:?}", instance)));
}
