use std::io::Read;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct RawBuffer {
    pub content: Vec<u8>,
}

impl RawBuffer {
    pub fn new(path: String) -> Self {

        let file = match File::open(&path) {
                Err(why) => panic!("buffer not available: {}", why),
                Ok(file) => file,
            };
        
        let buffer = BufReader::new(&file);

        let mut content: Vec<u8> = vec![];
        for byte in buffer.bytes() {
            content.push(byte.expect("error pushing"));
        }

        Self { content }
        }
    }

