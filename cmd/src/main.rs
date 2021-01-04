use std::fs::create_dir_all;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::env;

use rufy_core::Rufy;

fn main() {
    if !Path::new("project").exists() {
        create_dir_all("project").unwrap();
        let mut file = File::create(Path::new("project/main.lua")).unwrap();
        file.write_all(b"
function onInit()
    -- onInit is main function which ruth calls at startup
end").unwrap();
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return
    }
    let usage = args[1].as_str();
    match usage {
        "run" => run(),
        _ => help(),
    }
}

fn help() {
    println!("rufy --help\n");
    println!("USAGE: ");
    println!("\t run \trun project in debug mode")
}

fn run() {
    Rufy::init(String::from("project"));
}