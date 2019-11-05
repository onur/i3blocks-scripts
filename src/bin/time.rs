extern crate i3blocks;
extern crate time;

fn main() {
    i3blocks::Script::new().on_refresh(|instance| {
        time::now()
            .strftime(
                instance
                    .instance
                    .as_ref()
                    .unwrap_or(&"%Y-%m-%d %H:%M:%S".to_string()),
            )
            .ok()
            .map(|t| format!("{}", t))
    });
}
