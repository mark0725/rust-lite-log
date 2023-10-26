use lite_log::LiteLogger;

fn main() {
    LiteLogger::new().init().unwrap();

    log::warn!("This is an example message.");
}