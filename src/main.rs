use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::str;

static SEPARATOR: &str = "_____\n\n";

// https://stardict-4.sourceforge.net/StarDictFileFormat

fn main() {
    let mut args = env::args().skip(1);
    let output_path = args.next().expect("Output argument");
    let index_path = args.next().expect("Index argument");
    let dict_path = args.next().expect("Path argument");
    let syn_path = args.next();

    // Open files here, to fail early if we can’t
    let mut index_reader = BufReader::new(File::open(&index_path).expect("Open index"));
    let mut dict_file = File::open(&dict_path).expect("Open dictionary");
    let mut output_file = File::create(&output_path).expect("Open output");

    let mut number = [0; 4];
    let mut synonyms: HashMap<u32, Vec<String>> = HashMap::new();

    if let Some(syn_path) = syn_path {
        let mut syn_reader = BufReader::new(File::open(&syn_path).expect("Open synonyms"));
        let mut word = Vec::with_capacity(100);
        loop {
            word.clear();
            let nb = syn_reader
                .read_until(b'\0', &mut word)
                .expect("Read headword");
            if nb == 0 {
                break;
            }

            word.pop(); // remove '\0'
            let headword = str::from_utf8(&word).expect("Decode headword");

            syn_reader.read_exact(&mut number).expect("Read offset");
            let offset = u32::from_be_bytes(number);

            if let Some(v) = synonyms.get_mut(&offset) {
                v.push(headword.to_string());
            } else {
                synonyms.insert(offset, vec![headword.to_string()]);
            }
        }
    }

    let mut index_offset = 0;
    let mut write_buffer = String::with_capacity(1_005_000);
    let mut word = Vec::with_capacity(200);
    loop {
        word.clear();
        let nb = index_reader
            .read_until(b'\0', &mut word)
            .expect("Read headword");

        if nb < 2 {
            break;
        }

        word.pop(); // remove '\0'
        let headword = str::from_utf8(&word).expect("Decode headword");

        index_reader.read_exact(&mut number).expect("Read offset");
        let offset = u32::from_be_bytes(number);
        index_reader.read_exact(&mut number).expect("Read size");
        let size = u32::from_be_bytes(number) as usize;

        dict_file
            .seek(SeekFrom::Start(offset as u64))
            .expect("Seek offset");
        let mut data = vec![0u8; size];
        dict_file.read_exact(&mut data).expect("Read data");
        let definition = str::from_utf8(&data).expect("Decode definition");

        write_buffer.push_str(SEPARATOR);
        write_buffer.push_str(headword);
        if let Some(syns) = synonyms.get_mut(&index_offset) {
            for syn in syns {
                write_buffer.push('|');
                write_buffer.push_str(syn);
            }
            // Reduce memory usage and makes it faster for next iteration
            synonyms.remove(&index_offset);
        }
        write_buffer.push('\n');
        write_buffer.push_str(definition);
        write_buffer.push('\n');

        // This avoid using too much memory and doesn’t hurt performance
        if write_buffer.len() >= 1_000_000 {
            write!(output_file, "{}", write_buffer).expect("Write to output");
            write_buffer.clear();
        }

        index_offset += 1;
    }

    write!(output_file, "{}", write_buffer).expect("Write to output");
}
