use rufy_core::Rufy;

fn main() {
    match Rufy::new(String::from("project")) {
        Err(err) => println!("Err: {:?}", err),
        _ => {},
    }
}
