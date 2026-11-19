mod gui;
mod zuza;

fn main() {
    match gui::run() {
        Ok(_) => {}
        Err(msg) => println!("{:#}", msg),
    }
}
