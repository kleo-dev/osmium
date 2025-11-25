fn main() {
    let mut engine = osmium::init();
    engine.thread("Main.event", move || {});
}
