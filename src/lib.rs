use std::io::Read;
use std::fs::{read_dir, DirEntry, File};

fn read_file(entry: DirEntry) {
    let mut bib = File::open(entry.path()).unwrap();
    let mut buffer: [u8; 1] = [0; 1];
    bib.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    bib.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}

pub fn foo() {
    let path: &str = "data/input";
    for folder in read_dir(path).unwrap().flatten() { // unwrap() == expect(msg)
        for file in read_dir(folder.path()).unwrap().flatten() {
            read_file(file);
            break;
        }
        break;
    }
}

pub fn next_entry() -> u8 {
    13
}

#[cfg(test)]
mod tests {
    use crate::next_entry;
    #[test]
    fn it_works() {
        let result = next_entry();
        assert_eq!(result, 13);
    }
}
