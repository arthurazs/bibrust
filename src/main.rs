use bibrust::foo;

fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .write_style(env_logger::WriteStyle::Always)
        .init();
    foo();
}
