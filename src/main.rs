use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, Read, BufRead, BufReader, Seek, SeekFrom};

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().expect("Path argument");
    let syn_path = args.next();
    let mut number = [0; 4];
    let mut synonyms: HashMap<u32, Vec<String>> = HashMap::new();

    if let Some(syn_path) = syn_path {
        let mut syn_reader = BufReader::new(File::open(&syn_path)
                                                 .expect("Open synonyms"));
        loop {
            let mut word = Vec::new();
            let nb = syn_reader.read_until(b'\0', &mut word)
                               .expect("Read headword");
            if nb == 0 {
                break;
            }


            word.pop();

            let headword = String::from_utf8(word).expect("Decode headword");
            syn_reader.read_exact(&mut number).expect("Read offset");
            let offset = u32::from_be_bytes(number);

            if let Some(v) = synonyms.get_mut(&offset) {
                v.push(headword);
            } else {
                synonyms.insert(offset, vec![headword]);
            }
        }
    }

    let mut dict_reader = File::open(&path).expect("Open dictionary");
    let mut index_reader = BufReader::new(io::stdin()); 
    let mut index_offset = 0;

    loop {
        let mut word = Vec::new();
        let nb = index_reader.read_until(b'\0', &mut word)
                             .expect("Read headword");

        if nb < 2 {
            break;
        }

        word.pop();

        let mut headwords = vec![String::from_utf8(word).expect("Decode headword")];
        index_reader.read_exact(&mut number).expect("Read offset");
        let offset = u32::from_be_bytes(number) as u64;
        index_reader.read_exact(&mut number).expect("Read size");
        let size = u32::from_be_bytes(number) as usize;

        dict_reader.seek(SeekFrom::Start(offset)).expect("Seek offset");
        let mut data = vec![0u8; size];
        dict_reader.read_exact(&mut data).expect("Read data");
        let data = String::from_utf8(data).expect("Encode data");

        if let Some(mut v) = synonyms.remove(&index_offset) {
            headwords.append(&mut v);
        }

        println!("_____\n");
        println!("{}", headwords.join("|"));
        println!("{}", data);

        index_offset += 1;
    }
}
