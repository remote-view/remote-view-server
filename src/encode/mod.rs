use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct RawBuffer {
    pub content: String
}

impl RawBuffer {
    pub fn new(path: String) -> Self {

        let mut file = match File::open(&path) {
                Err(why) => panic!("buffer not available: {}", why),
                Ok(file) => file,
            };
        
        let mut content = String::from("");
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read buffer: {}", why),
            Ok(content) => content,
        };

        Self { content }
        }
    }

