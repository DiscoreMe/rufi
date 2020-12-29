use rufy_core::Rufy;

fn main() {
    let rufy = Rufy::new(String::from("project")).unwrap();
    rufy.run();
}
