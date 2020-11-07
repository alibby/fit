use std::fmt;
use std::io::Read;
use std::io::Cursor;
use std::fs::File;
use std::str;

use byteorder::{ LittleEndian, ReadBytesExt };

type Result<T> = std::result::Result<T,Error>;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}


#[derive(Debug)]
pub struct FileHeader {
    pub protocol_version: u8,
    pub profile_version: u16,
    pub data_size: u32,
    pub crc: u16,
}

impl FileHeader {
    fn new(buff : [u8; 14]) -> Result<FileHeader> {
        let fit_header = FileHeader {
            protocol_version: buff[1],
            profile_version: {
                let v = &buff[2..4].to_vec();
                let mut rdr = Cursor::new(v);
                rdr.read_u16::<LittleEndian>().unwrap()
            },
            data_size: {
                let v = &buff[4..8].to_vec();
                let mut rdr = Cursor::new(v);
                rdr.read_u32::<LittleEndian>().unwrap()
            },
            crc: {
                let v = &buff[12..14].to_vec();
                let mut rdr = Cursor::new(v);
                rdr.read_u16::<LittleEndian>().unwrap()
            }
        };

        Ok(fit_header)
    }

    pub fn read(rdr : &mut File) -> Result<FileHeader> {
        let mut buff = [0; 14];
        rdr.read(&mut buff).unwrap();

        FileHeader::check_header(buff).unwrap();
        FileHeader::new(buff)
    }

    fn check_header(buff : [u8; 14]) -> Result<()> {
        let header_size = &buff[0];
        let dotfit = str::from_utf8(&buff[8..12]).unwrap();
        if header_size != &14u8 {
            Err(Error {
                message: "invalid fit header.".to_string()
            })
        } else if dotfit != ".FIT" {
            Err(Error {
                message: "invalid fit header missing .FIT label".to_string()
            })
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_file_header() {
    let file = "files/2020-06-15-17-43-39.fit";
    let mut file = File::open(file).unwrap();
    let header = FileHeader::read(&mut file);
    println!("{:?}", header);

    assert!(false);
}

pub struct RecordHeader {
    pub header_byte: u8,
    pub normal: bool,
    pub compressed_timestamp: bool,
    pub data_message: bool,
    pub definition_message: bool,
    pub message_type: u8,
}


fn is_bit_set(byte : u8, bit : u8) -> bool {
    byte >> bit & 1 == 1
}

const NORMAL_BIT : u8 = 7u8;
const DEFINITION_MESSAGE_BIT : u8 = 6u8;
const MESSAGE_TYPE_BITS : u8 = 0xF;


impl RecordHeader {
    pub fn new(byte : u8) -> Result<RecordHeader> {
        Ok(RecordHeader {
            header_byte: byte,
            compressed_timestamp: is_bit_set(byte, NORMAL_BIT),
            normal: !is_bit_set(byte, NORMAL_BIT),
            definition_message: is_bit_set(byte, DEFINITION_MESSAGE_BIT),
            data_message: !is_bit_set(byte, DEFINITION_MESSAGE_BIT),
            message_type: byte & MESSAGE_TYPE_BITS,
        })

    }
}

impl fmt::Debug for RecordHeader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("FieldHeader")
            .field("header_byte", &format_args!("{:08b}", &self.header_byte))
            .field("normal", &self.normal)
            .field("compressed_timestamp", &self.compressed_timestamp)
            .field("data_message", &self.data_message)
            .field("definition_message", &self.definition_message)
            .field("message_type", &self.message_type)
            .finish()
    }
}

pub struct FieldDefinition {
    pub number : u8,
    pub size : u8,
    pub field_type: u8,
}

pub struct DefinitionMessage {
    pub reserved : u8,
    pub endian : u8,
    pub message_number : u16,
    pub field_count : u8,
}

