use log;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::{fs::File, path::PathBuf};

pub fn next_entry<R: Read>(mut bib: R) -> Cursor<Vec<u8>> {
    let mut buffer: [u8; 1] = [0; 1];
    let mut entry: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let mut found: bool = false;
    let mut counter: usize = 0;

    while bib.read(&mut buffer).unwrap() != 0 {
        entry.write(&buffer).unwrap();

        let open: bool = buffer[0] == 0x7b;
        let close: bool = buffer[0] == 0x7d;
        if !found {
            found = open;
        }

        counter += open as usize;
        counter -= close as usize;

        if found && counter == 0 {
            break;
        }
    }
    entry.seek(SeekFrom::Start(0)).unwrap(); // rewind to the start of the stream
    return entry;
}

pub fn parse_file(file_path: PathBuf) {
    log::info!("Parsing {}...", file_path.display());
    let bib: File = File::open(file_path).unwrap();
    // next_entry(&bib);
    let mut entry: Cursor<Vec<u8>> = next_entry(&bib);
    let mut buffer: String = String::new();
    entry.read_to_string(&mut buffer).unwrap();
    log::info!("Got next entry!");
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::next_entry;
    #[test]
    fn it_works() {
        let buff = Cursor::new(r"@article{1,
author = {Ahmad, Waqar and Hasan, Osman and Tahar, Sofi\`{e}ne},
title = {Formal reliability and failure analysis of ethernet based communication networks in a smart grid substation},
year = {2020},
issue_date = {Feb 2020},
publisher = {Springer-Verlag},
address = {Berlin, Heidelberg},
volume = {32},
number = {1},
issn = {0934-5043},
url = {https://doi.org/10.1007/s00165-019-00503-1},
doi = {10.1007/s00165-019-00503-1},
journal = {Form. Asp. Comput.},
month = {feb},
pages = {71–111},
numpages = {41},
keywords = {Theorem proving, Higher-order logic, Fault tree, Reliability block diagrams, Smart grid}
}

@article{2,");
        let expected: Cursor<Vec<u8>> = Cursor::new([64, 97, 114, 116, 105, 99, 108, 101, 123, 49, 44, 10, 97, 117, 116, 104, 111, 114, 32, 61, 32, 123, 65, 104, 109, 97, 100, 44, 32, 87, 97, 113, 97, 114, 32, 97, 110, 100, 32, 72, 97, 115, 97, 110, 44, 32, 79, 115, 109, 97, 110, 32, 97, 110, 100, 32, 84, 97, 104, 97, 114, 44, 32, 83, 111, 102, 105, 92, 96, 123, 101, 125, 110, 101, 125, 44, 10, 116, 105, 116, 108, 101, 32, 61, 32, 123, 70, 111, 114, 109, 97, 108, 32, 114,  101, 108, 105, 97, 98, 105, 108, 105, 116, 121, 32, 97, 110, 100, 32, 102, 97, 105, 108, 117, 114, 101, 32, 97, 110, 97, 108, 121, 115, 105, 115, 32, 111, 102, 32, 101, 116, 104, 101, 114, 110, 101, 116, 32, 98, 97, 115, 101, 100, 32, 99, 111, 109, 109, 117, 110, 105, 99, 97, 116, 105, 111, 110, 32, 110, 101, 116, 119, 111, 114, 107, 115, 32, 105, 110, 32, 97, 32, 115, 109, 97, 114, 116, 32, 103, 114, 105, 100, 32, 115, 117, 98, 115, 116, 97, 116, 105, 111, 110, 125, 44, 10, 121, 101, 97, 114, 32, 61, 32, 123, 50, 48, 50, 48, 125, 44, 10, 105, 115, 115, 117, 101, 95, 100, 97, 116, 101, 32, 61, 32, 123, 70, 101, 98, 32, 50, 48, 50, 48, 125, 44, 10, 112, 117, 98, 108, 105, 115, 104, 101, 114, 32, 61, 32, 123, 83, 112, 114, 105, 110, 103, 101, 114, 45, 86, 101, 114, 108, 97, 103, 125, 44, 10, 97, 100, 100, 114, 101, 115, 115, 32, 61, 32, 123, 66, 101, 114, 108, 105, 110, 44, 32, 72, 101, 105, 100, 101, 108, 98, 101, 114, 103, 125, 44, 10, 118, 111, 108, 117, 109, 101, 32, 61, 32, 123, 51, 50, 125, 44, 10, 110, 117, 109, 98, 101, 114, 32, 61, 32, 123, 49, 125, 44, 10, 105, 115, 115, 110, 32, 61, 32, 123, 48, 57, 51, 52, 45, 53, 48, 52, 51, 125, 44, 10, 117, 114, 108, 32, 61, 32, 123, 104, 116, 116, 112, 115, 58, 47, 47, 100, 111, 105, 46, 111, 114, 103, 47, 49, 48, 46, 49, 48, 48, 55, 47, 115, 48, 48, 49, 54, 53, 45, 48, 49, 57, 45, 48, 48, 53, 48, 51, 45, 49, 125, 44, 10, 100, 111, 105, 32, 61, 32, 123, 49, 48, 46, 49, 48, 48, 55, 47, 115, 48, 48, 49, 54, 53, 45, 48, 49, 57, 45, 48, 48, 53, 48, 51, 45, 49, 125, 44, 10, 106, 111, 117, 114, 110, 97, 108, 32, 61, 32, 123, 70, 111, 114, 109, 46, 32, 65, 115, 112, 46, 32, 67, 111, 109, 112, 117, 116, 46, 125, 44, 10, 109, 111, 110, 116, 104, 32, 61, 32, 123, 102, 101, 98, 125, 44, 10, 112, 97, 103, 101, 115, 32, 61, 32, 123, 55, 49, 226, 128, 147, 49, 49, 49, 125, 44, 10, 110, 117, 109, 112, 97, 103, 101, 115, 32, 61, 32, 123, 52, 49, 125, 44, 10, 107, 101, 121, 119, 111, 114, 100, 115, 32, 61, 32, 123, 84, 104, 101, 111, 114, 101, 109, 32, 112, 114, 111, 118, 105, 110, 103, 44, 32, 72, 105, 103, 104, 101, 114, 45, 111, 114, 100, 101, 114, 32, 108, 111, 103, 105, 99, 44, 32, 70, 97, 117, 108, 116, 32, 116, 114, 101, 101, 44, 32, 82, 101, 108, 105, 97, 98, 105, 108, 105, 116, 121, 32, 98, 108, 111, 99, 107, 32, 100, 105, 97, 103, 114, 97, 109, 115, 44, 32, 83, 109, 97, 114, 116, 32, 103, 114, 105, 100, 125, 10, 125].to_vec());
        let result = next_entry(buff);
        assert_eq!(result, expected);
    }
}
