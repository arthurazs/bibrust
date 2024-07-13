use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::{fs::File, path::PathBuf};

pub fn next_entry<R: Read>(bib: &mut R) -> Cursor<Vec<u8>> {
    let mut buffer: [u8; 1] = [0; 1];
    let mut entry: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let mut found: bool = false;
    let mut counter: usize = 0;

    while bib.read(&mut buffer).unwrap() != 0 {
        entry.write_all(&buffer).unwrap();

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
    entry
}

pub fn parse_file(file_path: PathBuf) {
    log::info!("Parsing {}...", file_path.display());
    let mut bib: File = File::open(file_path).unwrap();
    let mut entry: Cursor<Vec<u8>> = next_entry(&mut bib);
    let mut buffer: String = String::new();
    entry.read_to_string(&mut buffer).unwrap();
    log::info!("Got next entry!");
}

mod case_tests;
#[cfg(test)]
mod tests {
    use crate::case_tests::case1::Case;
    use crate::next_entry;
    use std::io::Cursor;
    use std::io::{Read, Seek, SeekFrom};
    const EMPTY_CHARS: [u8; 4] = [b'\t', b'\n', b'\r', b' '];

    trait Teller {
        fn tell(&mut self) -> u64;
        fn remaining_contents(&mut self) -> bool;
    }
    macro_rules! impl_Teller {
        (for $($t:ty),+) => {
            $(impl Teller for Cursor<$t> {
                fn tell(&mut self) -> u64 {
                    self.seek(SeekFrom::Current(0)).unwrap()
                }

                fn remaining_contents(&mut self) -> bool {
                    let cookie = self.tell();
                    let mut empty: bool = true;
                    let mut buffer: [u8; 1] = [0; 1];

                    while self.read(&mut buffer).unwrap() != 0 {
                        if !EMPTY_CHARS.contains(&buffer[0]) {
                            empty = false;
                            break;
                        }
                    }

                    self.seek(SeekFrom::Start(cookie)).unwrap();
                    return !empty;
                }
            })*
        };
    }
    impl_Teller!(for Vec<u8>, String);

    #[test]
    fn next_entry_cases() {
        for mut case in Case::new() {
            let mut entry: Cursor<Vec<u8>> = next_entry(&mut case.file);
            assert_eq!(entry.tell(), 0);
            assert!(entry.remaining_contents());
            assert_eq!(entry, case.expected_entry1);
            assert_eq!(case.file.tell(), case.expected_tell1);

            entry = next_entry(&mut case.file);
            assert_eq!(entry.tell(), 0);
            assert!(entry.remaining_contents());
            assert_eq!(entry, case.expected_entry2);
            assert_eq!(case.file.tell(), case.expected_tell2);

            entry = next_entry(&mut case.file);
            assert_eq!(entry.tell(), 0);
            assert!(!entry.remaining_contents());
            assert_eq!(entry, case.expected_entry3);
            assert_eq!(case.file.tell(), case.expected_tell3);

            entry = next_entry(&mut case.file);
            assert_eq!(entry.tell(), 0);
            assert!(!entry.remaining_contents());
            assert_eq!(entry, case.expected_entry4);
            assert_eq!(case.file.tell(), case.expected_tell4);
        }
    }
}
