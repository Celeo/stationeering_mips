use clap::Parser;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static COMMAND_ARGS: Lazy<HashMap<&'static str, u8>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("alias", 2);
    map.insert("define", 2);
    map.insert("move", 2);
    map.insert("s", 3);
    map.insert("l", 3);
    map.insert("j", 1);
    map.insert("yield", 0);
    map.insert("sub", 3);
    map.insert("div", 3);
    map.insert("add", 3);
    map.insert("sleep", 1);
    map
});

#[derive(Debug)]
enum Line {
    Empty,
    Label(String),
    Statement(String),
}

#[derive(Debug)]
struct Device {
    index: u8,
    values: HashMap<u8, f64>,
}

#[derive(Debug)]
struct Register {
    index: u8,
    value: f64,
}

#[derive(Debug)]
struct Machine {
    lines: Vec<Line>,
    current_line: u16,
    registers: [Register; 16],
    devices: [Device; 7],
}

#[derive(Debug, Parser)]
struct Options {
    //
}

fn main() {
    //
}
