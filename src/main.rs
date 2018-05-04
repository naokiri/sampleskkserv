///
/// Sample implementation of simple skk dictionary server
///
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate encoding;

use encoding::{Encoding, DecoderTrap, EncoderTrap};
use encoding::all::EUC_JP;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write, Read};
use std::net::{TcpListener, TcpStream};
use std::vec::Vec;


const ADDRESS: &str = "127.0.0.1:1178";
const SOFTWARE_NAME: &str = env!("CARGO_PKG_NAME");
const SOFTWARE_VERSION: &str = env!("CARGO_PKG_VERSION");


fn handle(stream: TcpStream, dict_map: &HashMap<String, String>) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    loop {
        let mut command_buffer = [0];
        match reader.read(&mut command_buffer) {
            Err(e) => {
                error!("Failed to read. Probably client disconnected. {}", e.description());
                break;
            }
            Ok(_) => {
                trace!("command {}", command_buffer[0]);
                let mut response = String::new();
                match &command_buffer {
                    b"0" => {
                        trace!("0: disconnect without any response");
                        break;
                    }
                    b"1" => {
                        let mut buffer = Vec::new();
                        reader.read_until(b' ', &mut buffer).expect("Failed to read midashi");
                        let mut message = String::new();
                        EUC_JP.decode_to(&buffer, DecoderTrap::Ignore, &mut message).expect("Failed to decode euc-jp");
                        let midashi = &*message.trim_right_matches(|x: char| x.is_ascii_control());
                        trace!("1midashi: '{}'", midashi);

                        match dict_map.get(midashi) {
                            None => {
                                trace!("entry not found");
                                response.push_str("4\n");
                            }
                            Some(kouho) => {
                                trace!("entry for midashi: {} found. returning: {}", midashi, kouho);
                                response.push_str(&format!("1{}\n", kouho));
                            }
                        }
                    }
                    b"2" => {
                        trace!("2: return 'Software Name.Major.Minor.Revision '");
                        response.push_str(&format!("{} {} ", SOFTWARE_NAME, SOFTWARE_VERSION));
                    }
                    b"3" => {
                        trace!("3: return 'Hostname:address1:address2:...:addressN: '");
                        response.push_str(&format!("{} ", ADDRESS));
                    }
                    b"4" => {
                        trace!("4midashi ");
                        // Unimplemented.
                        response.push_str("4\n");
                    }
                    _ => {
                        // FIXME: Not sure what is the defacto standard behaviour for unspecified request.
                        warn!("???: unspecified command. returning '0'.");
                        response.push_str("0\n");
                    }
                }
                writer.write(&EUC_JP.encode(&response, EncoderTrap::Ignore).expect("Failed to encode")).unwrap();
                let _ = writer.flush();
            }
        }
    }
}

/// read skk dict file into hashmap
/// Doesn't care about special entries.
fn build_table(dictfile: &File) -> HashMap<String, String> {
    let mut table = HashMap::new();
    let reader = BufReader::new(dictfile);

    for line in reader.lines() {
        match line {
            Err(e) => {
                error!("Error. Skippling line. Error: {}", e.description());
                ()
            }
            Ok(linestr) => {
                match linestr.find(";;") {
                    Some(0) => {
                        trace!("Skip comment line");
                    }
                    _ => {
                        trace!("Not an comment");
                        match linestr.find("/") {
                            None => {
                                trace!("No / in dict entry");
                            }
                            Some(place) => {
                                let linestr = &linestr;
                                let (midashi, kouho) = linestr.split_at(place);
                                trace!("midashi: '{}', kouho: '{}'", midashi, kouho);
                                table.insert(midashi.to_string(), kouho.trim_right_matches("\n").to_string());
                            }
                        }
                    }
                }
            }
        };
    }
    return table;
}


fn main() {
    env_logger::init();

    // "~/.sampleskkserv/SKK-JISYO.L.utf8
    let mut path = env::home_dir().unwrap();
    path.push(".sampleskkserv/");
    path.push("SKK-JISYO.L.utf8");


    let dict_map;
    {
        let file = match File::open(&path) {
            Err(e) => {
                panic!("Couldn't open file {}: {}", path.display(), e.description());
            }
            Ok(file) => file
        };
        dict_map = build_table(&file);
    }

    let listener = TcpListener::bind(ADDRESS).unwrap();


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                trace!("new client!");
                handle(stream, &dict_map);
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}