use log;
// use std::fs::{read_dir, DirEntry, File, ReadDir};
use std::fs::{read_dir, ReadDir};
// use std::io::{Cursor, Read, Seek, SeekFrom, Write};

// fn next_entry(mut bib: &File) -> Cursor<Vec<u8>> {
//     let mut buffer: [u8; 1] = [0; 1];
//     let mut entry: Cursor<Vec<u8>> = Cursor::new(Vec::new());
//     let mut found: bool = false;
//     let mut counter: usize = 0;
//
//     while bib.read(&mut buffer).unwrap() != 0 {
//         entry.write(&buffer).unwrap();
//
//         let open: bool = buffer[0] == 0x7b;
//         let close: bool = buffer[0] == 0x7d;
//         if !found {
//             found = open;
//         }
//
//         counter += open as usize;
//         counter -= close as usize;
//
//         if found && counter == 0 {
//             break;
//         }
//     }
//     entry.seek(SeekFrom::Start(0)).unwrap(); // rewind to the start of the stream
//     return entry;
// }

// fn read_file(dir_entry: DirEntry) {
//     let bib: File = File::open(dir_entry.path()).unwrap();
//     let mut entry: Cursor<Vec<u8>> = next_entry(&bib);
//     let mut buffer: String = String::new();
//     entry.read_to_string(&mut buffer).unwrap();
//     println!("<{}>", buffer);
// }

// fn iterate_base(mut read_dir: DirEntry) {
//     while let Some(file) = read_dir.next() {
//         log::info!("{file:?}");
//         break;
//     }
// }

fn iterate_root(mut read_dir: ReadDir) {
    while let Some(folder) = read_dir.next() {
        log::info!("{folder:?}");
        break;
    }
}

pub fn foo() {
    let path: &str = "data/input";
    // unwrap() == expect(msg)
    match read_dir(path) {
        Ok(correct_path) => {
            log::info!("{path}");
            iterate_root(correct_path);
        }
        Err(e) => println!("\"{path}\": {e}"),
    }
    // match read_dir(path) {
    //     Ok(mut folders) => {
    //         while let Some(folder) = folders.next() {
    //             println!("{:?}", folder);
    //         }
    //     }
    //     Err(e) => println!("\"{path}\": {e}"),
    // }
    // if let Ok(folder) = read_dir(path) {
    // for folder in read_dir(path).unwrap().flatten() {
    //     for file in read_dir(folder.path()).unwrap().flatten() {
    //         read_file(file);
    //         break;
    //     }
    //     break;
    // }
}

pub fn next_entrys() -> u8 {
    13
}

#[cfg(test)]
mod tests {
    use crate::next_entrys;
    #[test]
    fn it_works() {
        let result = next_entrys();
        assert_eq!(result, 13);
    }
}
