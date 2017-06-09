
use std::env::var;
use std::collections::HashMap;


pub struct Script {
    pub name: String,
    pub button: Option<u32>,
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub instance: String,
    on_refresh: Option<Box<Fn() -> String>>,
    on_click: HashMap<u32, Box<Fn() -> String>>,
}


impl Script {
    pub fn new() -> Script {
        Script {
            button: var("BLOCK_BUTTON").ok().and_then(|b| b.parse().ok()),
            x: var("BLOCK_X").ok().and_then(|b| b.parse().ok()),
            y: var("BLOCK_Y").ok().and_then(|b| b.parse().ok()),
            instance: var("BLOCK_INSTANCE").map(|b| b.to_owned()).unwrap_or(String::new()),
            name: var("BLOCK_name").map(|b| b.to_owned()).unwrap_or(String::new()),
            on_refresh: None,
            on_click: HashMap::new(),
        }
    }

    pub fn on_refresh<F: 'static>(mut self, f: F) -> Script
        where F: Fn() -> String
    {
        self.on_refresh = Some(Box::new(f));
        self
    }

    pub fn on_left_click<F: 'static>(mut self, f: F) -> Script
        where F: Fn() -> String
    {
        self.on_click.insert(1, Box::new(f));
        self
    }

    pub fn on_right_click<F: 'static>(mut self, f: F) -> Script
        where F: Fn() -> String
    {
        self.on_click.insert(3, Box::new(f));
        self
    }

    pub fn on_middle_click<F: 'static>(mut self, f: F) -> Script
        where F: Fn() -> String
    {
        self.on_click.insert(2, Box::new(f));
        self
    }

    pub fn on_scroll_up<F: 'static>(mut self, f: F) -> Script
        where F: Fn() -> String
    {
        self.on_click.insert(4, Box::new(f));
        self
    }

    pub fn on_scroll_down<F: 'static>(mut self, f: F) -> Script
        where F: Fn() -> String
    {
        self.on_click.insert(5, Box::new(f));
        self
    }

    // 1 => left click
    // 2 => middle click
    // 3 => right click
    // 4 => scroll up
    // 5 => scroll down
    pub fn on_click<F: 'static>(mut self, button: u32, f: F) -> Script
        where F: Fn() -> String
    {
        self.on_click.insert(button, Box::new(f));
        self
    }

    pub fn run(self) {
        let output =
            self.button.map_or(self.on_refresh.as_ref().map_or(String::new(), |f| f()),
                               |button| self.on_click.get(&button).map_or(String::new(), |f| f()));
        println!("{}", output);
    }
}
