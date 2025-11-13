use std::fs::{File,read_to_string};

#[derive(Debug)]
pub struct RawBuffer {
    pub content: String
}

impl RawBuffer {
    pub fn new() -> Self {
        Self {
            content: match File::open("/dev/fb0") {
                Err(why) => panic!("buffer nbt available"),
                Ok(file) => read_to_string(file).unwrap().lines(),
            },
        }
    }
    fn to_str(f: File) -> String {
        let mut res = String::new();
        for line in read_to_string(f).unwrap().lines() {
            res += line;
        }

        return res;
    }
}
