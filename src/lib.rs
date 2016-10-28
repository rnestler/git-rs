#[macro_use]
extern crate nom;

use nom::{IResult, be_u32, length_value};

pub struct IndexEntry {
    pub ctime_s: u32,
    pub ctime_ns: u32,
    pub mtime_s: u32,
    pub mtime_ns: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub file_size: u32,
    pub sha_1: [u8; 20],
    pub flags: u16,
}

pub struct IndexExtension<'a> {
    /// Signature starts with 'A'-'Z'
    pub signature: &'a [u8],
    pub data: &'a [u8],
}

pub fn parse_index_extension(input: &[u8]) -> IResult<&[u8], IndexExtension> {
    chain!(input,
           signature: take!(4) ~
           data: length_value,
           || {
               IndexExtension{
                   signature: signature,
                   data: data,
               }
           }
          )
}

/// The index header consists of 12 bytes:
///  * magic constant "DIRC"
///  * version
///  * number of index entries
#[derive(Debug)]
pub struct IndexHeader {
    /// BE version number
    version: u32,
    /// BE Number of index entries
    index_entries: u32,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], IndexHeader> {
    chain!(input,
           tag!("DIRC") ~
           version: be_u32 ~
           index_entries: be_u32 ,
           || {
               IndexHeader{
                   version: version,
                   index_entries: index_entries,
               }
           }
          )
}

pub struct Index<'a> {
    pub header: IndexHeader,
    pub entries: Vec<IndexEntry>,
    pub extensions: Vec<IndexExtension<'a>>,
    pub sha1_checksum: [u8; 20],
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_header() {
        let d = include_bytes!("../.git/modules/test_data/index");
        let header = parse_header(d).unwrap().1;
        assert_eq!(header.version, 2);
        assert_eq!(header.index_entries, 1);
    }

    #[test]
    fn test_parse_index_extension() {
        let d: &[u8] = b"ABCD\x01\x02";
        let extension = parse_index_extension(d).unwrap().1;

        assert_eq!(extension.signature, b"ABCD");
        assert_eq!(extension.data, &[2]);
    }
}
