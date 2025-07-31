use std::fs::File;

pub fn open_file(s: &str) -> File{
    let mut file = File::open(s);

    file.unwrap()


}