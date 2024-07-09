pub fn foo() {
    let string: &str = "@article{1,\n\
        author = {One, No},\n\
        title = {Validating}\n\
        }";
    println!("{string}");
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
