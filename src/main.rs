use std::env;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader, Seek, SeekFrom};

fn main() {
    let path = env::args().nth(1).expect("Read argument");
    let mut dict_reader = File::open(&path).expect("Open dictionary");
    let mut index_reader = BufReader::new(io::stdin()); 
    let mut number = [0; 4];

    loop {
        let mut word = Vec::new();
        let nb = index_reader.read_until(b'\0', &mut word)
                             .expect("Read headword");

        if nb == 0 {
            break;
        }

        word.pop();

        let headword = String::from_utf8(word).expect("Decode headword");
        index_reader.read_exact(&mut number).expect("Read offset");
        let offset = u32::from_be_bytes(number) as u64;
        index_reader.read_exact(&mut number).expect("Read size");
        let size = u32::from_be_bytes(number) as usize;

        dict_reader.seek(SeekFrom::Start(offset)).expect("Seek offset");
        let mut data = vec![0u8; size];
        dict_reader.read_exact(&mut data).expect("Read data");
        let data = String::from_utf8(data).expect("Encode data");

        println!("_____\n");
        println!("{}", headword);
        println!("{}", data);
    }
}
