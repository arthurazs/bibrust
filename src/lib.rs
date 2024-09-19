use std::fmt::Display;
use std::io::{Cursor, Read, Seek, SeekFrom, Write, BufReader};
use std::str::FromStr;
use std::{fs::File, path::PathBuf};

#[derive(Debug)]
pub struct Element {
    pub key: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub category: String,
    pub key: String,
    pub author: Vec<String>,
    pub r#abstract: String,
    pub title: String,
    pub journal: String,
    pub year: u16,
    pub keywords: Vec<String>,
    pub volume: String,
    pub number: String,
    pub pages: String,
    pub doi: String,
    pub issn: String,
    pub month: String,
    pub issue_date: String,
    pub publisher: String,
    pub address: String,
    pub url: String,
    pub numpages: u16,
    pub articleno: u16,
    pub note: String,
    pub affiliations: Vec<String>,
    pub author_keywords: Vec<String>,
    pub correspondence_address: Vec<String>,
    pub language: String,
    pub abbrev_source_title: String,
    pub publication_stage: String,
    pub source: String,
    pub coden: String,
    pub pmid: u32,
}
impl Entry {
    fn new(category: String, key: String) -> Entry {
        Entry {
            category,
            key,
            author: Vec::<String>::new(),
            r#abstract: String::new(),
            title: String::new(),
            journal: String::new(),
            year: 0,
            keywords: Vec::<String>::new(),
            volume: String::new(),
            number: String::new(),
            pages: String::new(),
            doi: String::new(),
            issn: String::new(),
            month: String::new(),
            issue_date: String::new(),
            publisher: String::new(),
            address: String::new(),
            url: String::new(),
            numpages: 0,
            articleno: 0,
            note: String::new(),
            affiliations: Vec::<String>::new(),
            author_keywords: Vec::<String>::new(),
            correspondence_address: Vec::<String>::new(),
            language: String::new(),
            abbrev_source_title: String::new(),
            publication_stage: String::new(),
            source: String::new(),
            coden: String::new(),
            pmid: 0,
        }
    }
}

fn next_entry<R: Read>(bib: &mut R) -> Cursor<Vec<u8>> {
    let mut buffer: [u8; 1] = [0; 1];
    let mut entry: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let mut found: bool = false;
    let mut counter: usize = 0;

    while bib.read(&mut buffer).unwrap() != 0 {
        entry.write_all(&buffer).unwrap();

        let open: bool = buffer[0] == b'{';
        let close: bool = buffer[0] == b'}';
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

fn get_category(entry: &mut Cursor<Vec<u8>>) -> String {
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

fn get_key(entry: &mut Cursor<Vec<u8>>) -> String {
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
    element_key
        .strip_prefix(",")
        .unwrap_or(&element_key)
        .trim()
        .to_lowercase()
}

fn get_element_value(entry: &mut Cursor<Vec<u8>>) -> String {
    let mut buffer: [u8; 1] = [0; 1];
    let mut element_key = String::new();
    let mut started: bool = false;
    let mut counter: usize = 0;

    while entry.read(&mut buffer).unwrap() != 0 {
        let open: bool = buffer[0] == b'{';
        if !started {
            if open {
                started = true;
                counter += 1;
            }
            continue;
        }
        let close: bool = buffer[0] == b'}';

        counter += open as usize;
        counter -= close as usize;

        if started && counter == 0 {
            break;
        }
        element_key.push(buffer[0] as char);
    }
    element_key.trim().to_string()
}

fn get_next_element(entry: &mut Cursor<Vec<u8>>) -> Element {
    Element {
        key: get_element_key(entry),
        value: get_element_value(entry),
    }
}

fn parse_int_element<T>(element: Element) -> T
where
    T: FromStr + From<u8>,
    <T as FromStr>::Err: Display,
{
    match element.value.parse::<T>() {
        Ok(parsed) => parsed,
        Err(reason) => {
            log::warn!(
                "Could not parse invalid {} {} [{reason}]",
                element.key,
                element.value
            );
            T::from(0)
        }
    }
}

pub fn parse_entry(entry: &mut Cursor<Vec<u8>>) -> Result<Entry, String> {
    let category: String = get_category(entry);
    if category.is_empty() {
        return Err(format!("Could not find category in entry"));
    }
    let key: String = get_key(entry);
    let mut parsed_entry = Entry::new(category.clone(), key);
    loop {
        let element: Element = get_next_element(entry);
        match element.key.as_str() {
            "author" => {
                parsed_entry.author = element
                    .value
                    .split("and")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
            "keywords" => {
                parsed_entry.keywords = element
                    .value
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
            "affiliations" => {
                parsed_entry.affiliations = element
                    .value
                    .split(";")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
            "author_keywords" => {
                parsed_entry.author_keywords = element
                    .value
                    .split(";")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
            "correspondence_address" => {
                parsed_entry.correspondence_address = element
                    .value
                    .split(";")
                    .map(|s| s.trim().to_string())
                    .collect();
            }
            "abstract" => {
                parsed_entry.r#abstract = element.value;
            }
            "title" => {
                parsed_entry.title = element.value;
            }
            "journal" => {
                parsed_entry.journal = element.value;
            }
            "year" => {
                parsed_entry.year = parse_int_element(element);
            }
            "volume" => {
                parsed_entry.volume = element.value;
            }
            "number" => {
                parsed_entry.number = element.value;
            }
            "pages" => {
                parsed_entry.pages = element.value;
            }
            "doi" => {
                parsed_entry.doi = element.value;
            }
            "issn" => {
                parsed_entry.issn = element.value;
            }
            "month" => {
                parsed_entry.month = element.value;
            }
            "issue_date" => {
                parsed_entry.issue_date = element.value;
            }
            "publisher" => {
                parsed_entry.publisher = element.value;
            }
            "address" => {
                parsed_entry.address = element.value;
            }
            "url" => {
                parsed_entry.url = element.value;
            }
            "numpages" => {
                parsed_entry.numpages = parse_int_element(element);
            }
            "articleno" => {
                parsed_entry.numpages = parse_int_element(element);
            }
            "note" => {
                parsed_entry.note = element.value;
            }
            "language" => {
                parsed_entry.language = element.value;
            }
            "abbrev_source_title" => {
                parsed_entry.abbrev_source_title = element.value;
            }
            "publication_stage" => {
                parsed_entry.publication_stage = element.value;
            }
            "source" => {
                parsed_entry.source = element.value;
            }
            "coden" => {
                parsed_entry.coden = element.value;
            }
            "pmid" => {
                parsed_entry.pmid = parse_int_element(element);
            }
            "type" => {
                if element.value.to_lowercase() != category {
                    log::warn!(
                        "Entry category \"{category}\" differs from element type \"{}\"",
                        element.value
                    );
                }
            }
            "}" | "" => {
                break;
            }
            _ => {
                log::warn!("Skipping unknown element: {}", element.key);
            }
        }
    }
    Ok(parsed_entry)
}

pub fn parse_file(file_path: PathBuf) -> usize {
    log::info!("Parsing {}...", file_path.display());
    let bib: File = File::open(file_path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(bib);
    let mut raw_entry: Cursor<Vec<u8>>;
    let mut counter: usize = 0;

    loop {
        raw_entry = next_entry(&mut reader);
        match parse_entry(&mut raw_entry) {
            Ok(_) => counter += 1,
            Err(_) => {
                log::info!("Reached end of file, read {counter} entries...");
                return counter;
            }
        }
    }
}

mod case_tests;
#[cfg(test)]
mod tests {
    use crate::case_tests::cases::{
        CaseGetElementKey, CaseGetElementValue, CaseGetKey, CaseGetNextElement, CaseParseEntry,
        ExpectedGetCategory, ExpectedNextEntry,
    };
    use crate::{
        get_category, get_element_key, get_element_value, get_key, get_next_element, next_entry,
        parse_entry, parse_int_element, Element,
    };
    use std::io::Cursor;
    use std::io::{Read, Seek, SeekFrom};
    const EMPTY_CHARS: [u8; 4] = *b"\t\n\r ";

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

    #[test]
    fn get_element_value_cases() {
        for mut case in CaseGetElementValue::new() {
            get_category(&mut case.entry);
            get_key(&mut case.entry);
            get_element_key(&mut case.entry);
            let element_value = get_element_value(&mut case.entry);
            assert_eq!(element_value, case.expected.value);
            assert_eq!(case.entry.tell(), case.expected.tell);
        }
    }

    #[test]
    fn get_next_element_cases() {
        for mut case in CaseGetNextElement::new() {
            get_category(&mut case.entry);
            get_key(&mut case.entry);
            let Element { key, value } = get_next_element(&mut case.entry);
            assert_eq!(key, case.expected.key);
            assert_eq!(value, case.expected.value);
            assert_eq!(case.entry.tell(), case.expected.tell);
        }
    }

    #[test]
    fn get_next_element_twice_cases() {
        for mut case in CaseGetNextElement::new_twice() {
            get_category(&mut case.entry);
            get_key(&mut case.entry);
            get_next_element(&mut case.entry);
            let Element { key, value } = get_next_element(&mut case.entry);
            assert_eq!(key, case.expected.key);
            assert_eq!(value, case.expected.value);
            assert_eq!(case.entry.tell(), case.expected.tell);
        }
    }

    #[test]
    fn parse_entry_cases() {
        for mut case in CaseParseEntry::new() {
            let parsed_entry = parse_entry(&mut case.entry);
            assert_eq!(parsed_entry, Ok(case.expected.parsed_entry));
            assert_eq!(case.entry.tell(), case.expected.tell);
        }
    }

    #[test]
    fn parse_int_positive() {
        let element: Element = Element {
            key: String::from("w/e"),
            value: String::from("1"),
        };
        assert_eq!(1, parse_int_element::<u8>(element));
    }

    #[test]
    fn parse_int_negative() {
        let element: Element = Element {
            key: String::from("w/e"),
            value: String::from("-1"),
        };
        assert_eq!(0, parse_int_element::<u8>(element));
    }

    #[test]
    fn parse_int_overflow() {
        let element: Element = Element {
            key: String::from("w/e"),
            value: u32::MAX.to_string(),
        };
        assert_eq!(0, parse_int_element::<u8>(element));
    }
}
