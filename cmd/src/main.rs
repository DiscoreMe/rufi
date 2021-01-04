use std::fs::create_dir_all;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::env;

use rufy_core::Rufy;

fn main() {
    if !Path::new("project").exists() {
        create_dir_all("project").unwrap();
        let mut file = File::create(Path::new( "project").join("main.lua")).unwrap();
        file.write_all(b"
function onInit()
    -- onInit is main function which ruth calls at startup
    -- run_scene(\"name\")
    -- Uncommoment the line above. It is start the first scene when the app starts
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
        "create" => create(args),
        _ => help(),
    }
}

fn help() {
    println!("rufy --help\n");
    println!("USAGE: ");
    println!("\t run \t\trun project in debug mode");
    println!("\t create [name]\tcreate new scene");
}

fn run() {
    Rufy::init(String::from("project"));
}

fn create(args: Vec<String>) {
    if args.len() < 3 {
        help();
        return
    }
    let name = args[2].as_str();

    create_dir_all(Path::new("project").join(name)).unwrap();
    let mut file = File::create(Path::new( "project").join(name).join("main.lua")).unwrap();
    file.write_all(b"
function onInit()
    -- onInit is main function which ruth calls scripts at scene
end").unwrap();
}