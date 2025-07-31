use std::fs::File;
use std::io::Write;
use std::path::Path;
 use std::fs;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
     let r = path.as_ref();
  
              fs::write(r, content).unwrap();




}
