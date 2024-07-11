use std::fs::{read_dir, DirEntry, ReadDir};

fn iterate_base(dir_entry: DirEntry) {
    if let Ok(mut dirs) = read_dir(dir_entry.path()) {
        while let Some(file_entry) = dirs.next() {
            match file_entry {
                Ok(file) => {
                    bibrust::parse_file(file.path());
                    break;
                }
                Err(e) => {
                    log::error!("{e}");
                    break;
                }
            }
        }
    }
}

fn iterate_root(mut dirs: ReadDir) {
    while let Some(dir_entry) = dirs.next() {
        match dir_entry {
            Ok(folder) => {
                log::info!("Opening {}...", folder.path().display());
                iterate_base(folder);
                break;
            }
            Err(e) => {
                log::error!("{e}");
                break;
            }
        }
    }
}

fn main() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .write_style(env_logger::WriteStyle::Always)
        .format_target(false)
        .format_module_path(true)
        .init();

    let path: &str = "data/input";
    match read_dir(path) {
        Ok(correct_path) => {
            log::info!("Opening folders...");
            iterate_root(correct_path);
        }
        Err(e) => println!("\"{path}\": {e}"),
    }
}
