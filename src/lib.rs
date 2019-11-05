use std::collections::HashMap;
use std::env::var;

#[derive(Debug)]
pub struct Instance {
    pub name: String,
    pub button: Option<u32>,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub instance: Option<String>,
}

pub struct Script(HashMap<u32, Box<Fn(&Instance) -> Option<String>>>);

impl Instance {
    fn new() -> Instance {
        Instance {
            button: var("BLOCK_BUTTON").ok().and_then(|b| b.parse().ok()),
            x: var("BLOCK_X").ok().and_then(|b| b.parse().ok()),
            y: var("BLOCK_Y").ok().and_then(|b| b.parse().ok()),
            instance: var("BLOCK_INSTANCE").ok().and_then(|b| {
                if b.is_empty() {
                    None
                } else {
                    Some(b.to_owned())
                }
            }),
            name: var("BLOCK_name")
                .map(|b| b.to_owned())
                .unwrap_or(String::new()),
        }
    }
}

macro_rules! script_fn {
    ($n:ident, $b:expr) => (
        pub fn $n<F: 'static>(mut self, f: F) -> Script
        where F: Fn(&Instance) -> Option<String>
        {
            self.0.insert($b, Box::new(f));
            self
        }
    );
}

impl Script {
    pub fn new() -> Script {
        Script(HashMap::new())
    }

    script_fn!(on_refresh, 0);
    script_fn!(on_left_click, 1);
    script_fn!(on_middle_click, 2);
    script_fn!(on_right_click, 3);
    script_fn!(on_scroll_up, 4);
    script_fn!(on_scroll_down, 5);

    pub fn on_click<F: 'static>(mut self, button: u32, f: F) -> Script
    where
        F: Fn(&Instance) -> Option<String>,
    {
        self.0.insert(button, Box::new(f));
        self
    }

    fn run(&self) {
        let instance = Instance::new();
        if let Some(output) = instance.button.map_or_else(
            || self.0.get(&0).and_then(|f| f(&instance)),
            |button| {
                self.0.get(&button).map_or_else(
                    || self.0.get(&0).and_then(|f| f(&instance)),
                    |f| f(&instance),
                )
            },
        ) {
            println!("{}", output);
        }
    }
}

impl Drop for Script {
    fn drop(&mut self) {
        self.run();
    }
}
