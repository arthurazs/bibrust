use std::{fs::{read_dir, DirEntry, ReadDir}, time::Instant};

fn iterate_base(dir_entry: DirEntry) -> usize {
    let mut counter: usize = 0;
    if let Ok(mut dirs) = read_dir(dir_entry.path()) {
        while let Some(file_entry) = dirs.next() {
            match file_entry {
                Ok(file) => {
                    counter += bibrust::parse_file(file.path());
                }
                Err(e) => {
                    log::error!("{e}");
                    break;
                }
            }
        }
    }
    return counter;
}

fn iterate_root(mut dirs: ReadDir) {
    let mut counter: usize = 0;
    let start = Instant::now();
    while let Some(dir_entry) = dirs.next() {
        match dir_entry {
            Ok(folder) => {
                log::info!("Opening {}...", folder.path().display());
                counter += iterate_base(folder);
            }
            Err(e) => {
                log::error!("{e}");
                break;
            }
        }
    }

    let elapsed: f64 = start.elapsed().as_micros() as f64 / 1000.0;
    let average: f64 = elapsed / counter as f64;
    log::info!("Took     {elapsed:8.3} ms to parse {counter} entries");
    log::info!("Averaged {average:8.3} ms per entry");
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
