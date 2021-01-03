use std::fs::create_dir_all;
use std::path::Path;
use std::fs::File;
use std::io::Write;

fn main() {
    if !Path::new("project").exists() {
        create_dir_all("project").unwrap();
        let mut file = File::create(Path::new("project/main.lua")).unwrap();
        file.write_all(b"
function onInit()
    -- onInit is main function which ruth calls at startup
end").unwrap();
    }

    Rufy::init(String::from("project"));
}