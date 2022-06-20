use std::fs::File;
use std::io;
use std::io::Read;

fn read_test_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?; // ? re-throw the Error
    Ok(s)
    /*
    ? used on Result is equvlant as:
    let mut f = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    */
}

#[test]
fn file_exist() {
    let text = read_test_from_file("io.txt").unwrap();
    assert_eq!("Hello", text);
}
