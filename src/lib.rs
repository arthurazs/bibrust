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

pub fn get_category(entry: &mut Cursor<Vec<u8>>) -> String {
    let mut buffer: [u8; 1] = [0; 1];
    let mut category = String::new();
    let mut found_at = false;
    while entry.read(&mut buffer).unwrap() != 0 {
        if !found_at {
            found_at = buffer[0] == b'@';
            continue;
        }
        if buffer[0] == b'{' {
            break;
        }
        category.push(buffer[0] as char);
    }
    category.to_lowercase()
}

pub fn get_key(entry: &mut Cursor<Vec<u8>>) -> String {
    let mut buffer: [u8; 1] = [0; 1];
    let mut key = String::new();
    while entry.read(&mut buffer).unwrap() != 0 {
        if buffer[0] == b',' {
            break;
        }
        key.push(buffer[0] as char);
    }
    key
}

fn get_element_key(entry: &mut Cursor<Vec<u8>>) -> String {
    let mut buffer: [u8; 1] = [0; 1];
    let mut element_key = String::new();
    while entry.read(&mut buffer).unwrap() != 0 {
        if buffer[0] == b'=' {
            break;
        }
        element_key.push(buffer[0] as char);
    }
    element_key.trim().to_lowercase()
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
    use crate::case_tests::cases::{
        CaseGetElementKey, CaseGetElementValue, CaseGetKey, ExpectedGetCategory, ExpectedNextEntry,
    };
    use crate::{get_category, get_element_key, get_element_value, get_key, next_entry};
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
        for mut case in ExpectedNextEntry::new() {
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

    #[test]
    fn get_category_cases() {
        for mut expected in ExpectedGetCategory::new() {
            let category: String = get_category(&mut expected.entry);
            assert_eq!(category, expected.category);
            assert_eq!(expected.entry.tell(), expected.tell)
        }
    }

    #[test]
    fn get_key_cases() {
        for mut case in CaseGetKey::new() {
            get_category(&mut case.entry);
            let key: String = get_key(&mut case.entry);
            assert_eq!(key, case.expected.value);
            assert_eq!(case.entry.tell(), case.expected.tell);
        }
    }

    #[test]
    fn get_element_key_cases() {
        for mut case in CaseGetElementKey::new() {
            get_category(&mut case.entry);
            get_key(&mut case.entry);
            let element_key = get_element_key(&mut case.entry);
            assert_eq!(element_key, case.expected.value);
            assert_eq!(case.entry.tell(), case.expected.tell);
        }
    }

}
