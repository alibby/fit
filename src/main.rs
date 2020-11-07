extern crate byteorder;

use std;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

mod fit;

use fit::{ Error, FileHeader, RecordHeader };

fn process_fit_file(file: &str) -> Result<(), Error> {
    println!("{}", file);
    let mut file = File::open(file).unwrap();
    let header = FileHeader::read(&mut file);
    println!("{:?}", header);

    let mut buff = [0u8; 1];

    file.read(&mut buff).unwrap();
    let rh = RecordHeader::new(buff[0]);
    println!("{:?}", rh);

    let mut buff = [0u8; 8];
    file.read(&mut buff).unwrap();
    println!("reserved                {:08b}", &buff[0]);
    println!("architecture            {:08b}", &buff[1]);
    println!("message number (lsb)    {:08b}", &buff[2]);
    println!("message number (msb)    {:08b}", &buff[3]);
    println!("number of fields        {:08b} ({})", &buff[4], &buff[4]);

    let mut buff = [0u8; 3];
    file.read(&mut buff).unwrap();
    println!("field definition number {:08b} ({})", &buff[0], &buff[0]);
    println!("size                    {:08b} ({})", &buff[1], &buff[1]);
    println!("base type               {:08b} ({})", &buff[2], &buff[2]);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let Some((_, files)) = args.split_first() {
        for file in files.iter() {
            let _ = process_fit_file(file);
        }
    }
    Ok(())
}

